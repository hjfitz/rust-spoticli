use tui::text::Spans;

use crate::ui::album_art::AlbumArtGenerator;

pub struct AlbumArtState {
    width: u32,
    remote_src: Option<String>,
    art: Option<Vec<Spans<'static>>>,
}

impl AlbumArtState {
    pub fn new() -> Self {
        Self {
            width: 50,
            remote_src: None,
            art: None,
        }
    }

    pub async fn try_update_by_src(&mut self, new_src: String) {
        let remote_src = self.remote_src.clone();
        match remote_src {
            Some(remote) => {
                if remote.ne(&new_src) {
                    self.update_by_src(new_src).await;
                }
            }
            None => {
                self.update_by_src(new_src).await;
            }
        }
    }

    async fn update_by_src(&mut self, new_src: String) {
        self.remote_src = Some(new_src);
        self.update().await;
    }

    pub async fn try_update_by_width(&mut self, new_width: u32) {
        if self.width.ne(&new_width) {
            self.width = new_width;
            self.update().await;
        }
    }

    async fn update(&mut self) {
        if self.remote_src.is_none() {
            return;
        }

        let remote = self.remote_src.as_ref().unwrap().clone();
        AlbumArtGenerator::fetch_art(remote).await;
        let generated_art = AlbumArtGenerator::generate_ascii_art(self.width);
        self.art = generated_art;
    }

    pub fn get_album_art(&self) -> Option<Vec<Spans<'static>>> {
        let art = self.art.clone();
        art
    }
}
