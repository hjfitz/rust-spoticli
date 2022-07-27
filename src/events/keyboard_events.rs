use super::types::SpotifyEvents;
use crossterm::event::{self, Event, KeyCode};
use std::io;
use tokio::sync::mpsc::UnboundedSender;

pub struct KeyboardEvents {
    tx: UnboundedSender<SpotifyEvents>,
}

impl KeyboardEvents {
    pub fn new(tx: UnboundedSender<SpotifyEvents>) -> Self {
        Self { tx }
    }
    pub fn poll(&self) -> Result<(), io::Error> {
        if let Event::Key(key) = event::read()? {
            match key.code {
                KeyCode::Down | KeyCode::Char('k') => {
                    self.tx.send(SpotifyEvents::NavigateDown);
                }
                KeyCode::Up | KeyCode::Char('j') => {
                    self.tx.send(SpotifyEvents::NavigateUp);
                }
                KeyCode::Left | KeyCode::Char('h') => {
                    self.tx.send(SpotifyEvents::NavigateLeft);
                }
                KeyCode::Right | KeyCode::Char('l') => {
                    self.tx.send(SpotifyEvents::NavigateRight);
                }
                KeyCode::Char('q') => {
                    self.tx.send(SpotifyEvents::Quit);
                }
                _ => {}
            }
        };

        Ok(())
    }
}
