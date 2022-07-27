use super::types::SpotifyEvents;
use crossterm::event::{self, Event, KeyCode};
use std::io;
use tokio::sync::mpsc::UnboundedSender;

type Transmitter = UnboundedSender<SpotifyEvents>;

pub struct KeyboardEvents {
    ui_tx: Transmitter,
}

impl KeyboardEvents {
    pub fn new(ui_tx: Transmitter) -> Self {
        Self { ui_tx }
    }
    pub fn poll(&self) -> Result<(), io::Error> {
        if let Event::Key(key) = event::read()? {
            match key.code {
                KeyCode::Down | KeyCode::Char('k') => {
                    self.ui_tx.send(SpotifyEvents::NavigateDown);
                }
                KeyCode::Up | KeyCode::Char('j') => {
                    self.ui_tx.send(SpotifyEvents::NavigateUp);
                }
                KeyCode::Left | KeyCode::Char('h') => {
                    self.ui_tx.send(SpotifyEvents::NavigateLeft);
                }
                KeyCode::Right | KeyCode::Char('l') => {
                    self.ui_tx.send(SpotifyEvents::NavigateRight);
                }
                KeyCode::Char('q') => {
                    self.ui_tx.send(SpotifyEvents::Quit);
                }
                KeyCode::Char(' ') | KeyCode::Enter => {
                    self.ui_tx.send(SpotifyEvents::PlayTrack);
                }
                _ => {}
            }
        };

        Ok(())
    }
}
