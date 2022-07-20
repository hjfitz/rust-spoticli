use std::time::SystemTime;

pub struct UpdateTicks {
    pub last_updated_at: SystemTime,
    pub interval_ms: i64,
    pub ms_since_update: i64, // probably going to change
}

impl UpdateTicks {
    pub fn new(interval: Option<i64>) -> Self {
        let interval_ms = interval.unwrap_or(1000);

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
        self.ms_since_update >= self.interval_ms
    }
}
