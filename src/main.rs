mod events;
mod services;
mod state;
mod types;
mod ui;
mod util;

use std::{io, thread, time::Duration};

use darkweb_dotenv::Dotenv;
use services::spotify_client::SpotifyClient;
use tui::{backend::CrosstermBackend, Terminal};

use crate::ui::player_ui::PlayerUi;

use crate::services::{oauth_server::OauthServer, spotify_adapter::SpotifyAdapter};
use crate::state::{
    playing_state::PlayingState, progress_state::ProgressBarState, state_adaptor::StateAdaptor,
};

use crate::events::keyboard_events::KeyboardEvents;

const THREAD_SLEEP_DURATION: std::time::Duration = Duration::from_millis(100);

#[tokio::main]
async fn main() -> Result<(), io::Error> {
    let mut dotenv = Dotenv::new();
    dotenv.load_env(".env", "APP_ENV", "dev").unwrap();

    let oauth_server = OauthServer::new();
    let spotify_access_token = oauth_server.get_access_token().await;

    let spotify = SpotifyClient::new(SpotifyAdapter::new(spotify_access_token));

    let playlists = spotify.fetch_playlists().await;

    let (playing_tx, mut playing_rx) = tokio::sync::mpsc::unbounded_channel();
    let (events_tx, mut events_rx) = tokio::sync::mpsc::unbounded_channel();
    let (data_tx, mut data_rx) = tokio::sync::mpsc::unbounded_channel();

    let polling_thread = tokio::spawn(async move {
        let progress_bar_state = ProgressBarState::new();
        let playing_state = PlayingState::new();
        let mut state_bridge =
            StateAdaptor::new(playing_state, progress_bar_state, spotify, playing_tx);
        loop {
            state_bridge.poll().await;
            let event = data_rx.try_recv();
            if event.is_ok() {
                state_bridge.handle_event(event.unwrap()).await;
            }
            thread::sleep(Duration::from_millis(200));
        }
    });

    let ui_thread = tokio::spawn(async move {
        let stdout = io::stdout();
        let backend = CrosstermBackend::new(stdout);
        let terminal = Terminal::new(backend).unwrap();
        let mut player_ui = PlayerUi::new(playlists, terminal, data_tx);
        player_ui.init_display().unwrap();
        loop {
            let kb_event = events_rx.try_recv();
            if kb_event.is_ok() {
                player_ui.handle_keyboard_events(kb_event.unwrap()).unwrap();
            }
            let data = playing_rx.try_recv();
            if data.is_ok() {
                player_ui.redraw(data.unwrap()).unwrap();
            }
            thread::sleep(Duration::from_millis(25));
        }
    });

    let events_thread = tokio::spawn(async move {
        let keyboard_events = KeyboardEvents::new(events_tx);
        loop {
            keyboard_events.poll().unwrap();
            thread::sleep(Duration::from_millis(25));
        }
    });

    polling_thread.await.unwrap();
    ui_thread.await.unwrap();
    events_thread.await.unwrap();

    Ok(())
}
