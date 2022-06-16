use crate::services::spotify_adapter::SpotifyAdapter;
use crate::types::full::now_playing::NowPlaying;
use crate::types::full::playback_state::PlaybackState;

pub struct SpotifyClient {
    adapter: SpotifyAdapter,
}

impl SpotifyClient {
    pub fn new(adapter: SpotifyAdapter) -> Self {
        SpotifyClient { adapter }
    }

    // @deprecated - use get_playback_state
    pub async fn get_now_playing(self) -> NowPlaying {
        self.adapter
            .get::<NowPlaying>("/me/player/currently-playing")
            .await
            .unwrap()
    }

    pub async fn get_playback_state(self) -> Option<PlaybackState> {
        let resp = self.adapter.get::<PlaybackState>("/me/player").await;

        match resp {
            Ok(r) => Some(r),
            Err(_) => None,
        }
    }

    // get_playlists(self) -> Result<Playlists> {

    // }

    // pub async fn perform_oauth_flow(&mut self) {
    //     let oauth_server = OauthServer::new();
    //     let callback_url = self.get_callback_url();

    //     let access_token = oauth_server.get_access_token(callback_url).await;
    //     self.adapter.set_access_token(access_token);
    // }
}
