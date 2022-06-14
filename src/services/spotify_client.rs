extern crate warp;

use std::env;

use crossbeam::channel;
use serde_derive::Deserialize;
use tokio::spawn;
use url_escape::encode_fragment;
use warp::{http, Filter};

#[derive(Deserialize)]
struct TokenAuth {
    access_token: String,
}

pub struct SpotifyClient {
    client_id: String,
    client_secret: String,
    scopes: Vec<String>,
    callback_url: String,
    is_initialised: bool,
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
            is_initialised: false,
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
        let (tx, rx) = channel::unbounded();

        let file_route = warp::get()
            .and(warp::path("callback"))
            .and(warp::path::end())
            // todo: hardcode this in the file and send str
            .and(warp::fs::file("./src/services/index.html"));

        let token_route = warp::post()
            .and(warp::path("token"))
            .and(warp::path::end())
            .and(warp::query::<TokenAuth>())
            .map(move |token: TokenAuth| {
                tx.send(token.access_token).unwrap();
                tx.send("kill".to_string()).unwrap();
                Ok(warp::reply::with_status("OK", http::StatusCode::OK))
            });

        println!("Spawning server for authentication");

        let handlers = token_route.or(file_route);

        let webserver_thread = tokio::spawn(async move {
            spawn(warp::serve(handlers).bind(([127, 0, 0, 1], 3000)))
                .await
                .unwrap();
        });

        let callback_url = self.get_callback_url();
        println!("Go to {} to login", callback_url);

        for msg in rx.recv() {
            match msg.as_str() {
                "kill" => webserver_thread.abort(),
                _ => self.access_token = msg,
            }
        }

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
