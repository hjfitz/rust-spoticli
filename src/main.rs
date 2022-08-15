mod events;
mod services;
mod state;
mod types;
mod ui;
mod util;

use std::{io, thread, time::Duration, sync::Arc};

use darkweb_dotenv::Dotenv;
use services::spotify_client::SpotifyClient;
use tokio::sync::Mutex;
use tui::{backend::CrosstermBackend, Terminal};

use crate::ui::player_ui::PlayerUi;

use crate::services::{oauth_server::OauthServer, spotify_adapter::SpotifyAdapter};
use crate::state::{
    album_art_state::AlbumArtState, playing_state::PlayingState, progress_state::ProgressBarState,
    state_adaptor::StateAdaptor,
};

use crate::events::keyboard_events::KeyboardEvents;

const THREAD_SLEEP_DURATION: std::time::Duration = Duration::from_millis(50);

#[tokio::main]
async fn main() -> Result<(), io::Error> {
    let mut dotenv = Dotenv::new();
    dotenv.load_env(".env", "APP_ENV", "dev").unwrap();

    let oauth_server = OauthServer::new();
    let spotify_access_token = oauth_server.get_access_token().await;

    let spotify = SpotifyClient::new(SpotifyAdapter::new(spotify_access_token));

    let playlists = spotify.fetch_playlists().await;

    let (events_tx, mut events_rx) = tokio::sync::mpsc::unbounded_channel();
    let (data_tx, mut data_rx) = tokio::sync::mpsc::unbounded_channel();

    let progress_bar_state = ProgressBarState::new();
    let playing_state = PlayingState::new();
    let art_state = AlbumArtState::new();
    let state_bridge = StateAdaptor::new(
        playing_state,
        progress_bar_state,
        spotify,
        art_state,
    );

    let state_bridge_mutex = Arc::new(Mutex::new(state_bridge));
    let state_bridge_handler = Arc::clone(&state_bridge_mutex);

    let polling_thread = tokio::spawn(async move {
        loop {
            let mut state = state_bridge_handler.lock().await;
            state.poll().await;
            drop(state);
            let event = data_rx.try_recv();
            if let Ok(spotify_event) = event {
                state_bridge_handler.lock().await.handle_event(spotify_event).await;
            }
            thread::sleep(THREAD_SLEEP_DURATION);
        }
    });

    let ui_thread = tokio::spawn(async move {
        let stdout = io::stdout();
        let backend = CrosstermBackend::new(stdout);
        let terminal = Terminal::new(backend).unwrap();
        let mut player_ui = PlayerUi::new(playlists, terminal, data_tx);
        player_ui.init_display().unwrap();
        loop {
            let new_kb_event = events_rx.try_recv();
            if let Ok(kb_event) = new_kb_event {
                player_ui.handle_keyboard_events(kb_event).unwrap();
            }
            let mut state = state_bridge_mutex.lock().await;
            let new_ui_state = state.get_state();
            player_ui.redraw(new_ui_state).unwrap();
            drop(state);
            thread::sleep(THREAD_SLEEP_DURATION);
        }
    });

    let events_thread = tokio::spawn(async move {
        let keyboard_events = KeyboardEvents::new(events_tx);
        loop {
            keyboard_events.poll().unwrap();
            thread::sleep(THREAD_SLEEP_DURATION);
        }
    });

    polling_thread.await.unwrap();
    ui_thread.await.unwrap();
    events_thread.await.unwrap();

    Ok(())
}
