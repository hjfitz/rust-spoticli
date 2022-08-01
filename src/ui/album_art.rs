use std::fs::{self, File};
use std::io::{copy, Cursor};
use std::path::Path;

use image::imageops::FilterType;
use image::GenericImageView;
use tui::style::Style;

use tui::style::Color;
use tui::text::{Span, Spans};

pub struct AlbumArtGenerator {}

impl AlbumArtGenerator {

    pub async fn fetch_art(remote: String) -> Option<warp::hyper::body::Bytes> {
        let resp = reqwest::get(remote).await;

        if resp.is_err() {
            return None
        }

        Some(resp.unwrap().bytes().await.unwrap())
    }

    pub async fn generate_ascii_art(remote: String, width: u32) -> Option<Vec<Spans<'static>>> {
        let raw_art_bytes = AlbumArtGenerator::fetch_art(remote).await?;

        let raw_art = image::load_from_memory(&raw_art_bytes).unwrap();

        let raw_art_resized = raw_art.resize_exact(width, width / 2, FilterType::Nearest);

        let mut album_art = vec![vec![]];
        let mut y = 0;

        for p in raw_art_resized.pixels() {
            if y != p.1 {
                y = p.1;
                album_art.push(vec![]);
            }

            let r = p.2 .0[0];
            let g = p.2 .0[1];
            let b = p.2 .0[2];

            // optional brightness calculation
            // let k = r as f32 * 0.3 + g as f32 * 0.59 + b as f32 * 0.11;
            // let character = ((k / 255.0) * 6f32).round() as usize;
            // let custom_char = pallete[character];

            let custom_char = '█';

            let char_style = Style::default().fg(Color::Rgb(r, g, b));
            let chr = Span::styled(String::from(custom_char), char_style);
            let len = album_art.len();
            let next_line = &mut album_art[len - 1];
            next_line.push(chr);
        }

        let processed_art = album_art
            .into_iter()
            .map(Spans::from)
            .collect::<Vec<Spans>>();

        Some(processed_art)


    }
}
