use crate::state::playlist_state::PlaylistState;
use crate::types::app::playlists::AppPlaylist;

use std::{io, thread, time::Duration};
use tui::{
    backend::CrosstermBackend,
    layout::{Constraint, Direction, Layout},
    widgets::{Block, Borders, Widget},
    Terminal,
};

use crossterm::{
    event::{self, DisableMouseCapture, EnableMouseCapture, Event, KeyCode},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};

pub struct PlayerUi {
    state: PlaylistState,
    playlists: Vec<AppPlaylist>,
    terminal: Terminal<CrosstermBackend<std::io::Stdout>>,
}

impl PlayerUi {
    pub fn new(
        playlists: Vec<AppPlaylist>,
        state: PlaylistState,
        terminal: Terminal<CrosstermBackend<std::io::Stdout>>,
    ) -> Self {
       
        Self {
            state,
            playlists,
            terminal,
        }
    }

    pub fn init_display(&mut self) -> Result<(), io::Error> {
        let mut stdout = io::stdout();
        execute!(stdout, EnterAlternateScreen, EnableMouseCapture)?;

        self.terminal.clear()?;
        self.terminal.draw(|f| {
            let size = f.size();
            let block = Block::default()
                .title("Block")
                .borders(Borders::ALL);
            f.render_widget(block, size);
        })?;

        thread::sleep(Duration::from_millis(5000));

        // restore terminal
        self.tear_down()?;

        Ok(())
    }

    pub fn tear_down(&mut self) -> Result<(), io::Error> {
        disable_raw_mode()?;
        execute!(
            self.terminal.backend_mut(),
            LeaveAlternateScreen,
            DisableMouseCapture
        )?;
        self.terminal.show_cursor()?;

        Ok(())
    }
}
