mod services;
mod types;

use darkweb_dotenv::Dotenv;
use services::spotify_client::SpotifyClient;

#[tokio::main]
async fn main() {
    let mut dotenv = Dotenv::new();
    dotenv.load_env(".env", "APP_ENV", "dev").unwrap();

    println!("Env initialised");

    let mut spotify = SpotifyClient::new();
    spotify.debug_env();
    println!("Client initialised");

    println!("Starting oauth flow");

    spotify.perform_oauth_flow().await;

    // let now_playing = spotify.get_now_playing().await;
    // println!("Currently playing: {:#?}", now_playing);

    // let playlists = spotify.get_playlists().await;

    let playback_state = spotify.get_playback_state().await;
    println!("Player state: {:#?}", playback_state);
}
