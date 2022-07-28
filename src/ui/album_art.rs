use std::fs::{self, File};
use std::io::{copy, Cursor};
use std::path::Path;

use ansi_term::ANSIStrings;
use ansi_term::Color::RGB;
use ansi_term::Style;
use glob::glob;
use image::imageops::FilterType;
use image::GenericImageView;

use ansi_to_tui::IntoText;
use tui::text::Text;

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

    pub fn generate_ascii_art(width: u32) -> Text<'static> {
        let fname_result = AlbumArtGenerator::find_album_art_file();

        if fname_result.is_none() {
            return Text::default();
        }

        let fname = fname_result.unwrap();
        let raw_img = image::open(fname);

        if raw_img.is_err() {
            panic!("{:?}", raw_img.err());
            return Text::default();
        }

        let img = raw_img.unwrap();

        let pallete: [char; 7] = [' ', '.', '/', '*', '#', '$', '@'];
        let mut y = 0;

        let mut art = vec![];
        let small_img = img.resize_exact(width, (width / 2), FilterType::Nearest);
        for p in small_img.pixels() {
            if y != p.1 {
                // art.push("\n".to_string());
                art.push(Style::new().paint("\n").to_string());
                y = p.1;
            }

            let r = p.2 .0[0] as f32;
            let g = p.2 .0[1] as f32;
            let b = p.2 .0[2] as f32;
            //luminosidade
            let k = r * 0.3 + g * 0.59 + b * 0.11;
            let character = ((k / 255.0) * (pallete.len() - 1) as f32).round() as usize;

            let custom_char = pallete[character];

            let coloured_char = RGB(r as u8, g as u8, b as u8)
                .paint(custom_char.to_string())
                .to_string();

            // art.push(custom_char.to_string());
            art.push(coloured_char);
        }

        // let painted = ANSIStrings(&art);

        let painted_text = art.join("").as_bytes().to_vec().into_text();

        painted_text.unwrap()

        // format!("{}", painted)
        // format!("{}", art.join(""))
    }
}
