mod services;
mod state;
mod types;
mod util;

use std::{io, thread, time::Duration};

use darkweb_dotenv::Dotenv;
use services::spotify_client::SpotifyClient;
use tui::{backend::CrosstermBackend, Terminal};

use crate::services::{
    oauth_server::OauthServer, player_ui::PlayerUi, spotify_adapter::SpotifyAdapter,
};
use crate::state::{
    playing_state::PlayingState, playlist_state::PlaylistState, progress_state::ProgressBarState,
    state_adaptor::StateAdaptor,
};

const THREAD_SLEEP_DURATION: std::time::Duration = Duration::from_millis(500);

#[tokio::main]
async fn main() -> Result<(), io::Error> {
    let mut dotenv = Dotenv::new();
    dotenv.load_env(".env", "APP_ENV", "dev").unwrap();

    let oauth_server = OauthServer::new();
    let spotify_access_token = oauth_server.get_access_token().await;

    let spotify = SpotifyClient::new(SpotifyAdapter::new(spotify_access_token));

    let playlists = spotify.fetch_playlists().await;

    let (playing_tx, mut playing_rx) = tokio::sync::mpsc::unbounded_channel();

    let polling_thread = tokio::spawn(async move {
        let progress_bar_state = ProgressBarState::new();
        let playing_state = PlayingState::new();
        let mut state_bridge =
            StateAdaptor::new(playing_state, progress_bar_state, spotify, playing_tx);
        loop {
            state_bridge.poll().await;
            thread::sleep(THREAD_SLEEP_DURATION);
        }
    });

    let ui_thread = tokio::spawn(async move {
        let stdout = io::stdout();
        let backend = CrosstermBackend::new(stdout);
        let terminal = Terminal::new(backend).unwrap();
        let playlist_state = PlaylistState::new();
        let mut player_ui = PlayerUi::new(playlists, playlist_state, terminal);
        player_ui.init_display().unwrap();
        loop {
            let data = playing_rx.recv().await;
            if data.is_some() {
                player_ui.redraw(data.unwrap()).unwrap();
            }
            thread::sleep(THREAD_SLEEP_DURATION);
        }
    });
    polling_thread.await.unwrap();
    ui_thread.await.unwrap();

    Ok(())
}
