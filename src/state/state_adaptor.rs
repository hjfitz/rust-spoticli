use super::update_ticks::UpdateTicks;
use crate::{PlayingState, ProgressBarState, SpotifyClient};
use tokio::sync::mpsc::UnboundedSender;

pub struct PlayerState {
    pub now_playing: String,
    pub time: String,
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
        if self.update_state.can_update() {
            self.update_live_player().await;
        }

        let state = PlayerState {
            now_playing: self.player_state.to_player_string(),
            time: self.progress_state.get_player_progress_seconds_str(),
        };

        // todo: handle
        self.tx.send(state);
    }
}
