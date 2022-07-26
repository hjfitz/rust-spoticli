use crate::state::playlist_state::PlaylistState;
use crate::types::app::playlists::AppPlaylist;

// use std::f64::is_nan;
use std::{io, thread, time::Duration};

use tui::{
    backend::CrosstermBackend,
    layout::{Alignment, Constraint, Direction, Layout, Rect},
    style::{Color, Modifier, Style},
    symbols,
    widgets::{Block, BorderType, Borders, LineGauge, List, ListItem, ListState, Paragraph},
    Terminal,
};

use crossterm::{
    event::{DisableMouseCapture, EnableMouseCapture, Event, KeyCode},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};

use crate::state::state_adaptor::PlayerState;

const PLAYER_DEFAULT_MARGIN: u16 = 1;

pub struct PlayerUi {
    playlist_state: ListState,
    playlists: Vec<AppPlaylist>,
    terminal: Terminal<CrosstermBackend<std::io::Stdout>>,
}

impl PlayerUi {
    pub fn new(
        playlists: Vec<AppPlaylist>,
        terminal: Terminal<CrosstermBackend<std::io::Stdout>>,
    ) -> Self {
        let mut playlist_state = ListState::default();
        playlist_state.select(Some(0));
        Self {
            playlist_state,
            playlists,
            terminal,
        }
    }

    pub fn init_display(&mut self) -> Result<(), io::Error> {
        // let mut stdout = io::stdout();
        // execute!(stdout, EnterAlternateScreen, EnableMouseCapture)?;

        self.terminal.clear()?;

        Ok(())
    }

    pub fn redraw(&mut self, state: PlayerState) -> Result<(), io::Error> {
        let playlist_items = self
            .playlists
            .iter()
            .map(|p| ListItem::new(p.name.clone()))
            .collect::<Vec<ListItem>>();

        let playlist_test_items = self.playlists[12]
            .items
            .iter()
            .map(|i| ListItem::new(i.track.name.clone()))
            .collect::<Vec<ListItem>>();
        self.terminal.draw(|frame| {
            let size = frame.size();
            let chunks = Layout::default()
                .direction(Direction::Vertical)
                .margin(PLAYER_DEFAULT_MARGIN)
                .constraints(
                    [
                        Constraint::Min(1),
                        Constraint::Length(4),
                        Constraint::Length(1),
                    ]
                    .as_ref(),
                )
                .split(size);

            let media_chunk = chunks[0];

            let playlist_list_chunk = Rect {
                x: media_chunk.x,
                y: media_chunk.y,
                height: media_chunk.height,
                width: media_chunk.width / 3,
            };

            let playlist_contents_chunk = Rect {
                x: media_chunk.x + (media_chunk.width / 3) + PLAYER_DEFAULT_MARGIN,
                y: media_chunk.y,
                height: media_chunk.height,
                width: media_chunk.width / 2,
            };
            let now_playing_content = format!("{}\n{}", state.now_playing, state.time);
            let now_playing_para = Paragraph::new(now_playing_content)
                .style(Style::default().fg(Color::LightCyan))
                .alignment(Alignment::Center)
                .block(
                    Block::default()
                        .borders(Borders::ALL)
                        .style(Style::default().fg(Color::White))
                        .border_type(BorderType::Plain),
                );

            let time_ratio_actual = (state.raw_time.seconds_elapsed as f64)
                / (state.raw_time.track_time_seconds as f64);

            let time_ratio = if time_ratio_actual.is_nan() {
                0f64
            } else {
                time_ratio_actual
            };

            let time_guage = LineGauge::default()
                .block(Block::default())
                .gauge_style(
                    Style::default()
                        .fg(Color::White)
                        .bg(Color::Black)
                        .add_modifier(Modifier::BOLD),
                )
                .line_set(symbols::line::THICK)
                .ratio(time_ratio);

            let playlist_list = List::new(playlist_items)
                .block(Block::default().title("Playlists").borders(Borders::ALL))
                .style(Style::default().fg(Color::White))
                .highlight_style(Style::default().add_modifier(Modifier::ITALIC))
                .highlight_symbol(" 👉 ");

            let playlist_items_list = List::new(playlist_test_items)
                .block(
                    Block::default()
                        .title("🍔🍕 full snack web development")
                        .borders(Borders::ALL),
                )
                .style(Style::default().fg(Color::White))
                .highlight_style(Style::default().add_modifier(Modifier::ITALIC))
                .highlight_symbol(" 👉  ");

            frame.render_widget(now_playing_para, chunks[1]);
            frame.render_widget(time_guage, chunks[2]);
            frame.render_stateful_widget(
                playlist_list,
                playlist_list_chunk,
                &mut self.playlist_state,
            );
            frame.render_widget(playlist_items_list, playlist_contents_chunk);
        })?;

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
