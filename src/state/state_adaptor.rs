use super::update_ticks::UpdateTicks;
use crate::state::progress_state::RawProgress;
use crate::{PlayingState, ProgressBarState, SpotifyClient};
use tokio::sync::mpsc::UnboundedSender;

pub struct PlayerState {
    pub now_playing: String,
    pub time: String,
    pub raw_time: RawProgress,
}

pub struct StateAdaptor {
    player_state: PlayingState,
    progress_state: ProgressBarState,
    spotify_client: SpotifyClient,
    tx: UnboundedSender<PlayerState>,
    update_state: UpdateTicks,
}

impl StateAdaptor {
    pub fn new(
        player_state: PlayingState,
        progress_state: ProgressBarState,
        spotify_client: SpotifyClient,
        tx: UnboundedSender<PlayerState>,
    ) -> Self {
        let update_state = UpdateTicks::new(Some(3000));
        Self {
            player_state,
            progress_state,
            spotify_client,
            tx,
            update_state,
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
            self.player_state.set_now_playing(
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
            );
        }
    }

    pub async fn poll(&mut self) {
        if self.player_state.can_update() {
            self.update_live_player().await;
        }

        if self.progress_state.can_update() {
            self.progress_state.bump_player_progress();
        }

        let state = PlayerState {
            now_playing: self.player_state.to_player_string(),
            time: self.progress_state.get_player_progress_seconds_str(),
            raw_time: self.progress_state.get_player_progress_seconds_raw(),
        };

        // todo: handle
        self.tx.send(state);
    }
}
