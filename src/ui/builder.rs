use crate::types::app::playlists::AppPlaylist;
use crate::types::full::playlist::Item;

use tui::{
    backend::CrosstermBackend,
    layout::{Alignment, Constraint, Direction, Layout, Rect},
    style::{Color, Modifier, Style},
    symbols,
    text::Span,
    widgets::{Block, BorderType, Borders, LineGauge, List, ListItem, Paragraph},
    Frame, 
};


use crate::state::state_adaptor::PlayerState;

const PLAYER_DEFAULT_MARGIN: u16 = 1;

pub struct PlayerAreas {
    pub progress: Rect,
    pub title: Rect,
    pub playlists: Rect,
    pub tracks: Rect,
    pub art: Rect,
}

pub struct Builder {}

impl Builder {
    fn create_styled_title(val: String, selected: bool) -> Span<'static> {
        match selected {
            true => Span::styled(val, Style::default().fg(Color::Green)),
            false => Span::styled(val, Style::default().fg(Color::Magenta)),
        }
    }

    fn create_list<'list>(
        items: Vec<ListItem<'list>>,
        title: Span<'list>,
        selected: bool,
    ) -> List<'list> {
        let hi_color = if selected {
            Color::LightGreen
        } else {
            Color::LightMagenta
        };
        List::new(items)
            .block(Block::default().title(title).borders(Borders::ALL))
            .style(Style::default().fg(hi_color))
            .highlight_style(Style::default().add_modifier(Modifier::ITALIC))
            .highlight_symbol(" 👉 ")
    }

    pub fn create_playlist_list(playlists: &[AppPlaylist], selected: bool) -> List {
        let playlist_items = playlists
            .iter()
            .map(|p| ListItem::new(p.name.clone()))
            .collect::<Vec<ListItem>>();

        let title = Builder::create_styled_title("Playlists".to_string(), selected);
        Builder::create_list(playlist_items, title, selected)
    }

    pub fn create_playlist_track_list(tracks: &[Item], title: String, selected: bool) -> List {
        let track_titles = tracks
            .iter()
            .map(|i| ListItem::new(i.track.name.clone()))
            .collect::<Vec<ListItem>>();

        let playlist_track_title = Builder::create_styled_title(title, selected);
        Builder::create_list(track_titles, playlist_track_title, selected)
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
            x: playlist_list_chunk.x + playlist_list_chunk.width + PLAYER_DEFAULT_MARGIN,
            y: media_chunk.y,
            height: media_chunk.height,
            width: (media_chunk.width / 3) - 2,
        };

        let now_playing_art = Rect {
            x: playlist_contents_chunk.x + playlist_contents_chunk.width + PLAYER_DEFAULT_MARGIN,
            y: media_chunk.y,
            height: media_chunk.height,
            width: (media_chunk.width / 3) - 1,
        };

        //let now_playing_art = Rect {
        //x: playlist_contents_chunk.x +

        PlayerAreas {
            playlists: playlist_list_chunk,
            tracks: playlist_contents_chunk,
            title: chunks[1],
            progress: chunks[2],
            art: now_playing_art,
        }
    }
}
