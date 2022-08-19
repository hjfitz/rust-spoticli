use crate::events::types::SpotifyEvents;
use crate::state::album_art_state::AlbumArtState;
use crate::state::progress_state::RawProgress;
use crate::{PlayingState, ProgressBarState, SpotifyClient};
use tui::text::Spans;

#[derive(PartialEq)]
pub struct PlayerState {
    pub now_playing: String,
    pub time: String,
    pub raw_time: RawProgress,
    pub album_art: Option<Vec<Spans<'static>>>,
}

pub struct StateAdaptor {
    player_state: PlayingState,
    progress_state: ProgressBarState,
    art_state: AlbumArtState,
    spotify_client: SpotifyClient,
}

impl StateAdaptor {
    pub fn new(
        player_state: PlayingState,
        progress_state: ProgressBarState,
        spotify_client: SpotifyClient,
        art_state: AlbumArtState,
    ) -> Self {
        Self {
            player_state,
            progress_state,
            spotify_client,
            art_state,
        }
    }

    async fn update_live_player(&mut self) {
        let now_playing = self.spotify_client.get_playback_state().await;
        if now_playing.is_some() {
            let playing = now_playing.unwrap();
            let track_seconds = playing.item.duration_ms / 1000;
            let listened_seconds = playing.progress_ms / 1000;
            self.progress_state
                .set_new_track(Some(listened_seconds), track_seconds);
            self.progress_state.set_is_playing(playing.is_playing);
            self.player_state
                .set_now_playing(
                    playing.item.name,
                    playing.item.album.name,
                    playing
                        .item
                        .album
                        .artists
                        .into_iter()
                        .map(|artists| artists.name)
                        .collect::<Vec<String>>()
                        .join(" "),
                )
                .await;
            let new_art_src = playing.item.album.images[0].url.clone();
            self.art_state.try_update_by_src(new_art_src).await;
        }
    }

    pub async fn handle_event(&mut self, event: SpotifyEvents) {
        match event {
            SpotifyEvents::StartTrack(track) => {
                self.spotify_client
                    .play_track(track.track_id, track.playlist_id)
                    .await;
            }
            SpotifyEvents::SetArtWidth(new_width) => {
                self.art_state.try_update_by_width(new_width).await;
            }
            _ => {}
        }
    }

    pub async fn poll(&mut self) {
        if self.player_state.can_update() {
            self.update_live_player().await;
        }

        if self.progress_state.can_update() {
            self.progress_state.bump_player_progress();
        }
    }


    pub fn get_state(&mut self) -> PlayerState {
        PlayerState {
            now_playing: self.player_state.to_player_string(),
            time: self.progress_state.get_player_progress_seconds_str(),
            raw_time: self.progress_state.get_player_progress_seconds_raw(),
            album_art: self.art_state.get_album_art(),
        }

    }
}
