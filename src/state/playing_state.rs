use tui::text::Text;

use crate::ui::album_art::AlbumArtGenerator;

use super::update_ticks::UpdateTicks;

pub struct PlayingState {
    title: String,
    album: String,
    artist: String,
    // id: String,
    // album_art: Option<Text>,
    update_state: UpdateTicks,
}

impl PlayingState {
    pub fn new() -> Self {
        let update_state = UpdateTicks::new(Some(3000));
        Self {
            title: "".to_string(),
            album: "".to_string(),
            artist: "".to_string(),
            // id: "".to_string(),
            // album_art: None,
            update_state,
        }
    }

    pub async fn set_now_playing(
        &mut self,
        title: String,
        album: String,
        artist: String,
        // id: String,
        // art_url: String, // todo: probably should use art url instead of ID
    ) {
        self.title = title;
        self.album = album;
        self.artist = artist;

        // if self.id.ne(&id) {
        //     AlbumArtGenerator::fetch_art(art_url)
        //     self.id = id;
        //     AlbumArtGenerator::clean();
        // }

        self.update_state.reset();
    }

    pub fn can_update(&mut self) -> bool {
        self.update_state.can_update()
    }

    pub fn to_player_string(&self) -> String {
        format!("{} - {} ({})", self.title, self.album, self.artist)
    }
}
