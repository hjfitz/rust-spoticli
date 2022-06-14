use std::env;
use url_escape::encode_fragment;

use super::oauth_server::OauthServer;

pub struct SpotifyClient {
    client_id: String,
    client_secret: String,
    scopes: Vec<String>,
    callback_url: String,
    access_token: String,
}

impl SpotifyClient {
    pub fn new() -> Self {
        let client_id = env::var("SPOTIFY_CLIENT_ID").unwrap();
        let client_secret = env::var("SPOTIFY_CLIENT_SECRET").unwrap();
        let callback_url = env::var("SPOTIFY_CALLBACK_URL").unwrap();

        let scopes_raw = env::var("SPOTIFY_SCOPES").unwrap();
        let scopes = Vec::from_iter(scopes_raw.split(',').map(|s| s.trim()).map(String::from));

        SpotifyClient {
            client_id,
            client_secret,
            callback_url,
            scopes,
            access_token: "".to_string(),
        }
    }

    fn get_callback_url(&self) -> String {
        let scopes = &self.scopes.join(" ");
        format!(
            "https://accounts.spotify.com/authorize?response_type=token&client_id={}&scope={}&redirect_uri={}",
            encode_fragment(&self.client_id).to_owned(),
            encode_fragment(scopes),
            encode_fragment(&self.callback_url),
        )
    }

    // this should eventually it in it's own OauthServer implementation
    pub async fn perform_oauth_flow(&mut self) {
        let oauth_server = OauthServer::new();
        let callback_url = self.get_callback_url();

        let access_token = oauth_server.get_access_token(callback_url).await;

        self.access_token = access_token.to_string();
        self.debug_self();
    }

    pub fn debug_env(&self) {
        let printable_scopes = self.scopes.join(", ");
        println!("SPOTIFY_CLIENT_ID: {}", self.client_id);
        println!("SPOTIFY_CLIENT_SECRET: {}", self.client_secret);
        println!("SPOTIFY_CLIENT_CALLBACK_URL: {}", self.callback_url);
        println!("SPOTIFY_CLIENT_SCOPES: {}", printable_scopes);
    }

    fn debug_self(&self) {
        println!("access token: {}", self.access_token);
    }
}
