mod services;
mod state;
mod types;
mod util;

use std::{thread, time::Duration};

use darkweb_dotenv::Dotenv;
use services::spotify_client::SpotifyClient;

use crate::services::{oauth_server::OauthServer, spotify_adapter::SpotifyAdapter};
use crate::state::playing_state::PlayingState;
use crate::state::progress_state::ProgressBarState;

const THREAD_SLEEP_DURATION: std::time::Duration = Duration::from_millis(500);

#[tokio::main]
async fn main() {
    let mut dotenv = Dotenv::new();
    dotenv.load_env(".env", "APP_ENV", "dev").unwrap();

    let oauth_server = OauthServer::new();
    let spotify_access_token = oauth_server.get_access_token().await;

    let spotify = SpotifyClient::new(SpotifyAdapter::new(spotify_access_token));

    // initialise everything
    // progress bar and playing state need different intervals; they work together. probably want an adapter impl to join these tbh
    let mut progress_bar_state = ProgressBarState::new();
    let mut playing_state = PlayingState::new();

    // playlist view doesn't really need any kind of timer. it's more involved internally though

    // this will probably wind up in an adapter class with a ref to each of the parameters and a single public 'update' method
    async fn update_live_player(
        spotify: &SpotifyClient,
        progress_bar_state: &mut ProgressBarState,
        playing_state: &mut PlayingState,
    ) {
        let now_playing = spotify.get_playback_state().await;
        if now_playing.is_some() {
            let playing = now_playing.unwrap();
            let track_seconds = playing.item.duration_ms / 1000;
            let listened_seconds = playing.progress_ms / 1000;
            progress_bar_state.set_new_track(Some(listened_seconds), track_seconds);
            playing_state.set_now_playing(
                playing.item.name,
                playing.item.album.name,
                playing
                    .item
                    .album
                    .artists
                    .into_iter()
                    .map(|artists| artists.name)
                    .collect::<Vec<String>>()
                    .join(" "),
            )
        }
    }

    println!("{}", progress_bar_state.get_player_progress_seconds_str());

    loop {
        if progress_bar_state.can_update() {
            progress_bar_state.bump_player_progress();
        }

        if playing_state.can_update() {
            update_live_player(&spotify, &mut progress_bar_state, &mut playing_state).await;
        }

        // instead of printing, we'll enter the terminal's alternate screen and build a UI
        // probably use crossbeam for this
        println!("{}", playing_state.to_player_string());
        println!("{}", progress_bar_state.get_player_progress_seconds_str());
        thread::sleep(THREAD_SLEEP_DURATION);
    }
}
