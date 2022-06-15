use serde_derive::Deserialize;
use tokio::spawn;
use warp::{http, Filter};

#[derive(Deserialize)]
struct TokenAuth {
    access_token: String,
}
pub struct OauthServer {}

impl OauthServer {
    // todo: move oauth config to this struct instead of spotify_client
    pub fn new() -> Self {
        Self {}
    }

    pub async fn get_access_token(&self, callback_url: String) -> String {
        let (tx, mut rx) = tokio::sync::mpsc::unbounded_channel();

        let redirect_page = include_str!("./index.html");

        let file_route = warp::get()
            .and(warp::path("callback"))
            .and(warp::path::end())
            .map(move || warp::reply::html(redirect_page));

        let token_route = warp::post()
            .and(warp::path("token"))
            .and(warp::path::end())
            .and(warp::query::<TokenAuth>())
            .map(move |token: TokenAuth| {
                tx.send(token.access_token).unwrap();
                Ok(warp::reply::with_status("OK", http::StatusCode::OK))
            });

        println!("Spawning server for authentication");

        let handlers = token_route.or(file_route);

        let webserver_thread = tokio::spawn(async move {
            spawn(warp::serve(handlers).bind(([127, 0, 0, 1], 3000)))
                .await
                .unwrap();
        });

        println!("Go to {} to login", callback_url);

        let access_token = rx.recv().await.unwrap();

        webserver_thread.abort();

        access_token
    }
}
