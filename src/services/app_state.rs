use std::time::{Duration, SystemTime};

use crate::util::time::seconds_to_time_string;

struct UpdateTicks {
    pub last_updated_at: SystemTime,
    pub interval_ms: i64,
    pub ms_since_update: i64, // probably going to change
}

impl UpdateTicks {
    pub fn new(interval: Option<i64>) -> Self {
        let interval_ms = match interval {
            Some(i) => i,
            None => 1000,
        };
        Self {
            last_updated_at: SystemTime::now(),
            interval_ms,
            ms_since_update: 0,
        }
    }

    pub fn reset(&mut self) {
        self.last_updated_at = SystemTime::now();
        self.ms_since_update = 0;
    }

    pub fn can_update(&mut self) -> bool {
        // bump time since update
        let ms_since_update = self
            .last_updated_at
            .elapsed()
            .unwrap()
            .as_millis()
            .try_into()
            .unwrap();

        self.ms_since_update = ms_since_update;
        println!("Milliseconds since the last update: {}", ms_since_update);

        self.ms_since_update >= self.interval_ms
    }
}

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

    // pub fn update() {}

    pub fn set_player_progress(&mut self, seconds: i64) {
        self.seconds_elapsed = seconds;
    }

    pub fn bump_player_progress(&mut self) {
        self.seconds_elapsed += 1;
        self.update_state.reset();
    }

    pub fn set_new_track(&mut self, cur_progress: Option<i64>, track_seconds: i64) {
        self.seconds_elapsed = match cur_progress {
            Some(t) => t,
            None => 0,
        };
        self.track_time_seconds = track_seconds;
    }

    pub fn get_player_progress_seconds_str(&self) -> String {
        let track_time = seconds_to_time_string(self.track_time_seconds);
        let listened_time = seconds_to_time_string(self.seconds_elapsed);

        let progress = format!("{}/{}", listened_time, track_time);

        progress
    }
}

// struct TrackState {
//     title: String,
//     album: String,
//     artist: String,
//     update_state: UpdateTicks,
// }

// impl TrackState {
//     pub fn new
// }

// struct AppNowPlaying {
//     title: String,
//     album: String,
//     // we'll concat the artists
//     artist: String,
// }
// pub struct AppState {
//     is_paused: bool,
//     now_playing: Option<AppNowPlaying>,
// }

// impl AppState {
//     pub fn new() -> Self {
//         Self {
//             now_playing: None,
//             is_paused: true,
//         }
//     }

//     pub fn set_now_playing(&mut self, title: String, album: String, artist: String, time: i64) {
//         let now_playing = AppNowPlaying {
//             title,
//             album,
//             artist,
//         };
//         self.now_playing = Some(now_playing);
//     }
// }
