mod services;
mod types;
mod util;

use std::{thread, time::Duration};

use darkweb_dotenv::Dotenv;
use services::spotify_client::SpotifyClient;

use crate::services::{
    app_state::ProgressBarState, oauth_server::OauthServer, spotify_adapter::SpotifyAdapter,
};

const THREAD_SLEEP_DURATION: std::time::Duration = Duration::from_millis(500);

#[tokio::main]
async fn main() {
    let mut dotenv = Dotenv::new();
    dotenv.load_env(".env", "APP_ENV", "dev").unwrap();

    let oauth_server = OauthServer::new();
    let spotify_access_token = oauth_server.get_access_token().await;

    let spotify = SpotifyClient::new(SpotifyAdapter::new(spotify_access_token));

    // initialise everything
    let now_playing = spotify.get_playback_state().await;
    let mut progress_bar_state = ProgressBarState::new();

    if now_playing.is_some() {
        let playing = now_playing.unwrap();
        let track_seconds = playing.item.duration_ms / 1000;
        let listened_seconds = playing.progress_ms / 1000;
        progress_bar_state.set_new_track(Some(listened_seconds), track_seconds);
    }

    println!("{}", progress_bar_state.get_player_progress_seconds_str());

    loop {
        if progress_bar_state.can_update() {
            progress_bar_state.bump_player_progress();
            // println!("{}", progress_bar_state.get_player_progress_seconds_str());
        }
        thread::sleep(THREAD_SLEEP_DURATION);
    }

    // get now playing

    // store the UI in a state struct

    // eventually set up loop
    // create thread for fetching now playing. do a loop
    // as above for player progress
    // 'app: loop {
    //     // draw the ui every millisecond
    // }
}
