mod services;

use darkweb_dotenv::Dotenv;
use services::spotify_client::SpotifyClient;

#[tokio::main]
async fn main() {
    let mut dotenv = Dotenv::new();
    dotenv.load_env(".env", "APP_ENV", "dev").unwrap();

    println!("Env initialised");

    let mut client = SpotifyClient::new();
    client.debug_env();
    println!("Client initialised");

    println!("Starting oauth flow");

    client.perform_oauth_flow().await;
}
