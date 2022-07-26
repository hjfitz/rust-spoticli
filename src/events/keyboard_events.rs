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
                KeyCode::Down => {
                    self.tx.send(SpotifyEvents::NavigateDown);
                }
                KeyCode::Up => {
                    self.tx.send(SpotifyEvents::NavigateUp);
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
