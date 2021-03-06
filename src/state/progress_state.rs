use crate::state::update_ticks::UpdateTicks;
use crate::util::time::seconds_to_time_string;

pub struct ProgressBarState {
    track_time_seconds: i64,
    seconds_elapsed: i64,
    is_paused: bool,
    update_state: UpdateTicks,
}

impl ProgressBarState {
    pub fn new() -> Self {
        let update_state = UpdateTicks::new(Some(1000));
        Self {
            track_time_seconds: 0,
            seconds_elapsed: 0,
            update_state,
            is_paused: true,
        }
    }

    pub fn can_update(&mut self) -> bool {
        // !self.is_paused && self.update_state.can_update()
        self.update_state.can_update()
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

    pub fn get_player_progress_seconds_str(&self) -> String {
        let track_time = seconds_to_time_string(self.track_time_seconds);
        let listened_time = seconds_to_time_string(self.seconds_elapsed);

        let progress = format!("{}/{}", track_time, listened_time);

        progress
    }
}
