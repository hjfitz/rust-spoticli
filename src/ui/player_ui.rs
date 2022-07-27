use crate::state::playlist_state::PlaylistState;
use crate::types::app::playlists::AppPlaylist;

// use std::f64::is_nan;
use std::{io, process, thread, time::Duration};

use tui::{
    backend::CrosstermBackend,
    layout::{Alignment, Constraint, Direction, Layout, Rect},
    style::{Color, Modifier, Style},
    symbols,
    widgets::{Block, BorderType, Borders, LineGauge, List, ListItem, ListState, Paragraph},
    Terminal,
};

use crossterm::{
    event::{DisableMouseCapture, EnableMouseCapture},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};

use crate::events::types::SpotifyEvents;

use super::builder::{Builder, PlayerAreas};

use crate::state::state_adaptor::PlayerState;

#[derive(PartialEq)]
enum SelectedPane {
    Playlist,
    Tracks,
}

pub struct PlayerUi {
    playlist_state: ListState,
    playlist_track_states: Vec<ListState>,
    playlists: Vec<AppPlaylist>,
    terminal: Terminal<CrosstermBackend<std::io::Stdout>>,
    selected_pane: SelectedPane,
}

impl PlayerUi {
    pub fn new(
        playlists: Vec<AppPlaylist>,
        terminal: Terminal<CrosstermBackend<std::io::Stdout>>,
    ) -> Self {
        let mut playlist_state = ListState::default();
        playlist_state.select(Some(0));
        let mut playlist_track_states = vec![];

        // initialise states for cursor in each playlist
        for _ in 0..playlists.len() {
            let mut track_state = ListState::default();
            track_state.select(Some(0));
            playlist_track_states.push(track_state);
        }

        Self {
            playlist_state,
            playlists,
            terminal,
            playlist_track_states,
            selected_pane: SelectedPane::Tracks,
        }
    }

    pub fn init_display(&mut self) -> Result<(), io::Error> {
        // let mut stdout = io::stdout();
        // execute!(stdout, EnterAlternateScreen, EnableMouseCapture)?;
        enable_raw_mode()?;

        self.terminal.clear()?;

        Ok(())
    }

    pub fn redraw(&mut self, state: PlayerState) -> Result<(), io::Error> {
        let playlist_list = Builder::create_playlist_list(&self.playlists);
        let playlist_idx = self.playlist_state.selected().unwrap_or(0);
        let playlist_items_list = Builder::create_playlist_track_list(
            &self.playlists[playlist_idx].items,
            self.playlists[playlist_idx].name.clone(),
        );
        let now_playing_para = Builder::create_now_playing_para(&state);
        let time_gauge = Builder::create_progress_bar(
            state.raw_time.seconds_elapsed as f64,
            state.raw_time.track_time_seconds as f64,
        );

        self.terminal.draw(|frame| {
            let PlayerAreas {
                playlists,
                tracks,
                title,
                progress,
            } = Builder::create_container(frame);

            frame.render_widget(now_playing_para, title);
            frame.render_widget(time_gauge, progress);
            frame.render_stateful_widget(playlist_list, playlists, &mut self.playlist_state);
            frame.render_stateful_widget(
                playlist_items_list,
                tracks,
                &mut self.playlist_track_states[playlist_idx],
            );
        })?;

        Ok(())
    }

    pub fn handle_keyboard_events(&mut self, event: SpotifyEvents) -> Result<(), io::Error> {
        match self.selected_pane {
            SelectedPane::Playlist => {
                match event {
                    SpotifyEvents::NavigateDown => {
                        // let iterator =
                        let cur_state = &mut self.playlist_state;
                        let cur_idx = cur_state.selected().unwrap_or(0);
                        let next_idx = if cur_idx + 1 >= self.playlists.len() {
                            cur_idx
                        } else {
                            cur_idx + 1
                        };

                        cur_state.select(Some(next_idx));
                        return Ok(());
                    }
                    SpotifyEvents::NavigateUp => {
                        let cur_idx = self.playlist_state.selected().unwrap_or(0);
                        let next_idx = if cur_idx == 0 { cur_idx } else { cur_idx - 1 };

                        self.playlist_state.select(Some(next_idx));
                    }
                    _ => {}
                }
            }
            SelectedPane::Tracks => {
                match event {
                    SpotifyEvents::NavigateDown => {
                        // let iterator =
                        let playlist_tracks_state_idx = self.playlist_state.selected().unwrap_or(0);
                        let cur_state = &mut self.playlist_track_states[playlist_tracks_state_idx];
                        let cur_idx = cur_state.selected().unwrap_or(0);
                        let next_idx = if cur_idx + 1
                            >= self.playlists[playlist_tracks_state_idx].items.len()
                        {
                            0
                        } else {
                            cur_idx + 1
                        };

                        cur_state.select(Some(next_idx));
                        return Ok(());
                    }
                    SpotifyEvents::NavigateUp => {
                        let playlist_tracks_state_idx = self.playlist_state.selected().unwrap_or(0);
                        let cur_state = &mut self.playlist_track_states[playlist_tracks_state_idx];
                        let cur_idx = cur_state.selected().unwrap_or(0);
                        let next_idx = if cur_idx == 0 {
                            self.playlists[playlist_tracks_state_idx].items.len() - 1
                        } else {
                            cur_idx - 1
                        };

                        cur_state.select(Some(next_idx));
                    }
                    _ => {}
                }
            }
        }
        match event {
            SpotifyEvents::Quit => {
                self.tear_down()?;
                process::exit(0x0100);
            }
            SpotifyEvents::NavigateLeft => {
                self.selected_pane = SelectedPane::Playlist;
            }
            SpotifyEvents::NavigateRight => {
                self.selected_pane = SelectedPane::Tracks;
            }
            _ => return Ok(()),
        }
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
