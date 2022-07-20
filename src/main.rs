mod services;
mod state;
mod types;
mod util;

use std::{thread, time::Duration};

use darkweb_dotenv::Dotenv;
use services::spotify_client::SpotifyClient;

use crate::services::{oauth_server::OauthServer, spotify_adapter::SpotifyAdapter};
use crate::state::{
    playing_state::PlayingState, progress_state::ProgressBarState, state_adaptor::StateAdaptor,
};

const THREAD_SLEEP_DURATION: std::time::Duration = Duration::from_millis(500);

#[tokio::main]
async fn main() {
    let mut dotenv = Dotenv::new();
    dotenv.load_env(".env", "APP_ENV", "dev").unwrap();

    let oauth_server = OauthServer::new();
    let spotify_access_token = oauth_server.get_access_token().await;

    let spotify = SpotifyClient::new(SpotifyAdapter::new(spotify_access_token));

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
        loop {
            let data = playing_rx.recv().await.unwrap();
            println!("{}", data.now_playing);
            println!("{}", data.time);
            thread::sleep(THREAD_SLEEP_DURATION);
        }
    });
    polling_thread.await.unwrap();
    ui_thread.await.unwrap();
}
