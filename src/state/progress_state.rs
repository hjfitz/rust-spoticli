use crate::state::update_ticks::UpdateTicks;
use crate::util::time::seconds_to_time_string;

pub struct ProgressBarState {
    track_time_seconds: i64,
    seconds_elapsed: i64,
    is_playing: bool,
    update_state: UpdateTicks,
}

pub struct RawProgress {
    pub track_time_seconds: i64,
    pub seconds_elapsed: i64,
}

impl ProgressBarState {
    pub fn new() -> Self {
        let update_state = UpdateTicks::new(Some(1000));
        Self {
            track_time_seconds: 0,
            seconds_elapsed: 0,
            update_state,
            is_playing: true,
        }
    }

    pub fn can_update(&mut self) -> bool {
        self.is_playing && self.update_state.can_update()
        // self.update_state.can_update()
    }

    pub fn bump_player_progress(&mut self) {
        // main update loop will progress to the next track
        if self.seconds_elapsed < self.track_time_seconds {
            self.seconds_elapsed += 1;
            self.update_state.reset();
        }
    }

    pub fn set_new_track(&mut self, cur_progress: Option<i64>, track_seconds: i64) {
        self.seconds_elapsed = cur_progress.unwrap_or(0);
        self.track_time_seconds = track_seconds;
    }

    pub fn set_is_playing(&mut self, playing: bool) {
        self.is_playing = playing;
    }

    pub fn get_player_progress_seconds_str(&self) -> String {
        let track_time = seconds_to_time_string(self.track_time_seconds);
        let listened_time = seconds_to_time_string(self.seconds_elapsed);

        let progress = format!("{}/{}", listened_time, track_time);

        progress
    }

    pub fn get_player_progress_seconds_raw(&self) -> RawProgress {
        RawProgress {
            track_time_seconds: self.track_time_seconds,
            seconds_elapsed: self.seconds_elapsed,
        }
    }
}
