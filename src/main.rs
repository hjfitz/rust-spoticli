mod services;
mod types;

use darkweb_dotenv::Dotenv;
use services::spotify_client::SpotifyClient;

use crate::services::{oauth_server::OauthServer, spotify_adapter::SpotifyAdapter};

#[tokio::main]
async fn main() {
    let mut dotenv = Dotenv::new();
    dotenv.load_env(".env", "APP_ENV", "dev").unwrap();

    let oauth_server = OauthServer::new();

    let spotify_access_token = oauth_server.get_access_token().await;

    let spotify = SpotifyClient::new(SpotifyAdapter::new(spotify_access_token));

    let playback_state = spotify.get_playback_state().await;
    println!("Player state: {:#?}", playback_state);
}
