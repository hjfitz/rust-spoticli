use std::fs::{self, File};
use std::io::{copy, Cursor};
use std::path::Path;

use image::imageops::FilterType;
use image::GenericImageView;
use tui::style::Style;

use tui::style::Color;
use tui::text::{Span, Spans};

pub struct AlbumArtGenerator {}

// ugly ugly hack
const ART_LOCATIONS: [&str; 3] = [
    "/tmp/spoticli-art.png",
    "/tmp/spoticli-art.jpg",
    "/tmp/spoticli-art.jpeg",
];

impl AlbumArtGenerator {
    fn cleanup() {
        for file in ART_LOCATIONS {
            fs::remove_file(file);
        }
    }

    fn find_album_art_file() -> Option<String> {
        for file in ART_LOCATIONS {
            if Path::new(file).exists() {
                return Some(file.to_string());
            }
        }

        return None;
    }

    pub async fn fetch_art(remote_url: String) -> Result<String, ()> {
        let response = reqwest::get(remote_url).await.unwrap();

        let content_type = response
            .headers()
            .get("content-type")
            .unwrap()
            .to_str()
            .unwrap()
            .to_string();

        let extension = content_type.replace("image/", "");

        let output_fname = format!("/tmp/spoticli-art.{}", extension);

        AlbumArtGenerator::cleanup();

        let mut dest = File::create(output_fname.clone()).unwrap();

        let mut content = Cursor::new(response.bytes().await.unwrap());
        copy(&mut content, &mut dest).unwrap();

        Ok(output_fname)
    }

    pub fn generate_ascii_art(width: u32) -> Option<Vec<Spans<'static>>> {
        let fname_result = AlbumArtGenerator::find_album_art_file();

        if fname_result.is_none() {
            return None;
        }

        let fname = fname_result.unwrap();

        // should probably store in memory whenever we get a new track playing
        // could probably store chars with escape codes in the file once we download and just
        // decode that every render of this. Can put this on to a new thread
        let raw_img = image::open(fname);

        if raw_img.is_err() {
            return None;
        }

        let img = raw_img.unwrap();

        //let pallete: [char; 7] = [' ', '.', '/', '*', '#', '$', '@'];
        let mut y = 0;

        let small_img = img.resize_exact(width, width / 2, FilterType::Nearest);

        let mut album_art = vec![vec![]];
        for p in small_img.pixels() {
            if y != p.1 {
                y = p.1;
                album_art.push(vec![]);
            }

            let r = p.2 .0[0];
            let g = p.2 .0[1];
            let b = p.2 .0[2];
            // brightness
            //let k = r as f32 * 0.3 + g as f32 * 0.59 + b as f32 * 0.11;
            //let character = ((k / 255.0) * 6f32).round() as usize;
            //let custom_char = pallete[character];
            let custom_char = '█';

            let char_style = Style::default().fg(Color::Rgb(r, g, b));
            let chr = Span::styled(String::from(custom_char), char_style);
            let len = album_art.len();
            let next_line = &mut album_art[len - 1];
            next_line.push(chr);
        }

        let processed_art = album_art
            .into_iter()
            .map(|l| Spans::from(l))
            .collect::<Vec<Spans>>();

        Some(processed_art)
    }
}
