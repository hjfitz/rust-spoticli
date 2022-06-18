use super::update_ticks::UpdateTicks;

pub struct PlayingState {
    title: String,
    album: String,
    artist: String,
    update_state: UpdateTicks,
}

impl PlayingState {
    pub fn new() -> Self {
        let update_state = UpdateTicks::new(Some(3000));
        Self {
            title: "".to_string(),
            album: "".to_string(),
            artist: "".to_string(),
            update_state,
        }
    }

    pub fn set_now_playing(&mut self, title: String, album: String, artist: String) {
        self.title = title;
        self.album = album;
        self.artist = artist;
        self.update_state.reset();
    }

    pub fn can_update(&mut self) -> bool {
        self.update_state.can_update()
    }

    pub fn to_player_string(&self) -> String {
        format!("{} - {} ({})", self.title, self.album, self.artist)
    }
}
