use crossbeam::channel;
use serde_derive::Deserialize;
use tokio::spawn;
use warp::{http, Filter};

#[derive(Deserialize)]
struct TokenAuth {
    access_token: String,
}
pub struct OauthServer {}

impl OauthServer {
    pub fn new() -> Self {
        Self {}
    }

    fn get_implicit_routes() {}

    pub async fn get_access_token(&self, callback_url: String) -> String {
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

        println!("Go to {} to login", callback_url);

        let mut access_token = String::new();
        for msg in rx.recv() {
            match msg.as_str() {
                "kill" => webserver_thread.abort(),
                _ => access_token = msg,
            }
        }
        access_token
    }
}
