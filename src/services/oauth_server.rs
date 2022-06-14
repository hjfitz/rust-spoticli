use crate::SpotifyClient;
pub struct OauthServer {
    port: i32,
    client: Option<SpotifyClient>,
}

impl OauthServer {
    pub fn new() -> Self {
        Self {
            port: 3000,
            client: None,
        }
    }

    pub fn start(&self) {
        if self.client.is_none() {
            panic!("Unable to find spotify client");
        }
    }

    pub fn stop(&self) {}
}
