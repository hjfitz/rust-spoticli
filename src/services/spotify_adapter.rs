use std::fmt::Debug;

const SPOTIFY_API_BASE: &'static str = "https://api.spotify.com/v1";

#[derive(Clone)]
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

    fn get_api_url(path: &str) -> String {
        SPOTIFY_API_BASE.to_owned() + path
    }

    pub async fn get<T: serde::de::DeserializeOwned>(&self, pathname: &str) -> Result<T, String> {
        let full_api_url = SpotifyAdapter::get_api_url(pathname);

        let resp_raw = self
            .http_client
            .get(full_api_url)
            .bearer_auth(&self.access_token)
            .send()
            .await;

        if resp_raw.is_err() {
            println!("{}", resp_raw.unwrap_err());
            println!("Unable to make request to {}", pathname);
            return Err(format!("Unable to make request to {}", pathname));
        }

        let resp = resp_raw.unwrap().text().await;

        if resp.is_err() {
            return Err("Unable to parse response to text".to_string());
        }

        let resp_data = resp.unwrap();

        let parsed_response = serde_json::from_str(&resp_data);

        if parsed_response.is_err() {
            println!("{}", resp_data.clone());
            panic!("{:?}", parsed_response.err());
            return Err("Unable to parse response".to_string());
        }

        Ok(parsed_response.unwrap())
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
