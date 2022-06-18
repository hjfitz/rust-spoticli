use crate::services::spotify_adapter::SpotifyAdapter;
use crate::types::full::playback_state::PlaybackState;

#[derive(Clone)]
pub struct SpotifyClient {
    adapter: SpotifyAdapter,
}

impl SpotifyClient {
    pub fn new(adapter: SpotifyAdapter) -> Self {
        Self { adapter }
    }

    pub async fn get_playback_state(&self) -> Option<PlaybackState> {
        let resp = self.adapter.get::<PlaybackState>("/me/player").await;

        match resp {
            Ok(r) => Some(r),
            Err(_) => None,
        }
    }

    // todo: types and ensure parsing ok
    // get_playlists(self, user_id: String) -> Result<PlaylistResponseDTO> {
    //     let pathname = format!("/users/{}/playlists", user_id);
    //     let playlists = self.adapter.get::<Vec<Playlists>>("/users/");

    // }

    // get_playlist_content(self, playlist_name: String) -> ??? {
    //     let pathname = format!("/playlists/{}/tracks", playlist_name)
    //}
}
