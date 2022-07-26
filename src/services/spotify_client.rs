use crate::services::spotify_adapter::SpotifyAdapter;
use crate::types::app::playlists::AppPlaylist;
use crate::types::full::playback_state::PlaybackState;
use crate::types::full::playlist::Playlist;
use crate::types::full::playlists::Playlists;
use crate::types::full::user::SpotifyUser;

#[derive(Clone)]
pub struct SpotifyClient {
    adapter: SpotifyAdapter,
}

impl SpotifyClient {
    pub fn new(adapter: SpotifyAdapter) -> Self {
        Self { adapter }
    }

    pub async fn get_playback_state(&self) -> Option<PlaybackState> {
        let player_state = self.adapter.get::<PlaybackState>("/me/player").await;

        player_state.ok()
    }

    // todo: types and ensure parsing ok
    pub async fn get_playlists(&self, user_id: String) -> Option<Playlists> {
        let pathname = format!("/users/{}/playlists", user_id);
        let playlists = self.adapter.get::<Playlists>(&pathname).await;

        playlists.ok()
    }

    pub async fn get_playlist_content(&self, playlist_name: &String) -> Option<Playlist> {
        let pathname = format!("/playlists/{}/tracks", playlist_name);
        let playlist = self.adapter.get::<Playlist>(&pathname).await;

        playlist.ok()
    }

    pub async fn get_user(&self) -> Option<SpotifyUser> {
        let me = self.adapter.get::<SpotifyUser>("/me").await;

        me.ok()
    }

    pub async fn fetch_playlists(&self) -> Vec<AppPlaylist> {
        let me = self.get_user().await.unwrap();
        let playlists = self.get_playlists(me.id).await.unwrap();

        let mut playlist_data = vec![];

        for playlist in playlists.items.into_iter() {
            println!("Fetching {} ({})", playlist.name, playlist.id);
            let cur_playlist = self.get_playlist_content(&playlist.id).await;
            if cur_playlist.is_some() {
                let hydrated_playlist = AppPlaylist {
                    id: playlist.id,
                    name: playlist.name,
                    items: cur_playlist.unwrap().items,
                };
                playlist_data.push(hydrated_playlist);
            }
        }

        playlist_data
    }
}
