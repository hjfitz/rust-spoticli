use crate::types::app::playlists::AppPlaylist;
use crate::types::full::playlist::Item;

use tui::{
    backend::CrosstermBackend,
    layout::{Alignment, Constraint, Direction, Layout, Rect},
    style::{Color, Modifier, Style},
    symbols,
    text::Span,
    widgets::{Block, BorderType, Borders, LineGauge, List, ListItem, ListState, Paragraph},
    Frame, Terminal,
};

use crossterm::{
    event::{DisableMouseCapture, EnableMouseCapture, Event, KeyCode},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};

use crate::state::state_adaptor::PlayerState;

const PLAYER_DEFAULT_MARGIN: u16 = 1;

pub struct PlayerAreas {
    pub progress: Rect,
    pub title: Rect,
    pub playlists: Rect,
    pub tracks: Rect,
}

pub struct Builder {}

impl Builder {
    fn create_list(items: Vec<ListItem>, title: &str) -> List {
        List::new(items)
            .block(
                Block::default()
                    .title(Span::styled(title, Style::default().fg(Color::Green)))
                    .borders(Borders::ALL),
            )
            .style(Style::default().fg(Color::White))
            .highlight_style(Style::default().add_modifier(Modifier::ITALIC))
            .highlight_symbol(" 👉 ")
    }

    pub fn create_playlist_list(playlists: &Vec<AppPlaylist>) -> List {
        let playlist_items = playlists
            .iter()
            .map(|p| ListItem::new(p.name.clone()))
            .collect::<Vec<ListItem>>();

        Builder::create_list(playlist_items, "Playlists")
    }

    pub fn create_playlist_track_list(tracks: &Vec<Item>, title: String) -> List {
        let track_titles = tracks
            .iter()
            .map(|i| ListItem::new(i.track.name.clone()))
            .collect::<Vec<ListItem>>();

        let playlist_title = title.as_str();
        Builder::create_list(track_titles, playlist_title)
    }

    pub fn create_progress_bar(seconds_elapsed: f64, track_time: f64) -> LineGauge<'static> {
        let time_ratio_actual = seconds_elapsed / track_time;

        let time_ratio = if time_ratio_actual.is_nan() {
            0f64
        } else {
            time_ratio_actual
        };

        LineGauge::default()
            .block(Block::default())
            .gauge_style(
                Style::default()
                    .fg(Color::White)
                    .bg(Color::Black)
                    .add_modifier(Modifier::BOLD),
            )
            .line_set(symbols::line::THICK)
            .ratio(time_ratio)
    }

    pub fn create_now_playing_para(state: &PlayerState) -> Paragraph {
        let now_playing_content = format!("{}\n{}", state.now_playing, state.time);
        Paragraph::new(now_playing_content)
            .style(Style::default().fg(Color::LightCyan))
            .alignment(Alignment::Center)
            .block(
                Block::default()
                    .borders(Borders::ALL)
                    .style(Style::default().fg(Color::White))
                    .border_type(BorderType::Plain),
            )
    }

    pub fn create_container(frame: &Frame<CrosstermBackend<std::io::Stdout>>) -> PlayerAreas {
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

        PlayerAreas {
            playlists: playlist_list_chunk,
            tracks: playlist_contents_chunk,
            title: chunks[1],
            progress: chunks[2],
        }
    }
}
