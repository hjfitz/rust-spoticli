const SPOTIFY_API_BASE: &'static str = "https://api.spotify.com/v1";

pub struct SpotifyAdapter {
    access_token: String,
    http_client: reqwest::Client,
}

impl SpotifyAdapter {
    pub fn new(access_token: String) -> Self {
        let http_client = reqwest::Client::new();

        Self {
            access_token,
            http_client,
        }
    }

    pub fn set_access_token(&mut self, access_token: String) {
        self.access_token = access_token;
    }

    fn get_api_url(path: &str) -> String {
        SPOTIFY_API_BASE.to_owned() + &path.to_string()
    }

    pub async fn get<T: serde::de::DeserializeOwned>(&self, pathname: &str) -> Result<T, ()> {
        let full_api_url = SpotifyAdapter::get_api_url(pathname);

        let resp = self
            .http_client
            .get(full_api_url)
            .bearer_auth(&self.access_token)
            .send()
            .await
            .unwrap()
            .text()
            .await
            .unwrap();

        let parsed_response: T = serde_json::from_str(&resp).unwrap();

        Ok(parsed_response)
    }

    pub async fn get_raw(&self, pathname: &str) -> Result<String, ()> {
        let full_api_url = SpotifyAdapter::get_api_url(pathname);

        let raw_response = self
            .http_client
            .get(full_api_url)
            .bearer_auth(&self.access_token)
            .send()
            .await
            .unwrap()
            .text()
            .await
            .unwrap();

        Ok(raw_response)
    }
}
