pub struct PlaylistState {
    selected_playlist: i64,
    cursor_position: i64,
}

impl PlaylistState {
    pub fn new() -> Self {
        Self {
            selected_playlist: 0,
            cursor_position: 0,
        }
    }
}
