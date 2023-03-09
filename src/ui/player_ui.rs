use std::{io, process};

use tokio::sync::mpsc::UnboundedSender;
use tui::{
    backend::CrosstermBackend,
    style::Style,
    widgets::{Block, Borders, ListState, Paragraph},
    Terminal,
};

use crossterm::{
    event::DisableMouseCapture,
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, LeaveAlternateScreen},
};

use super::builder::{Builder, PlayerAreas};

use crate::events::types::SpotifyEvents;
use crate::state::state_adaptor::PlayerState;
use crate::state::update_ticks::UpdateTicks;
use crate::types::app::event_types::NewSong;
use crate::types::app::playlists::AppPlaylist;

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
    data_tx: UnboundedSender<SpotifyEvents>,
    redraw_ticks: UpdateTicks,
    state: Option<PlayerState>,
}

impl PlayerUi {
    pub fn new(
        playlists: Vec<AppPlaylist>,
        terminal: Terminal<CrosstermBackend<std::io::Stdout>>,
        data_tx: UnboundedSender<SpotifyEvents>,
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

        let redraw_ticks = UpdateTicks::new(None);

        Self {
            playlist_state,
            playlists,
            terminal,
            data_tx,
            playlist_track_states,
            redraw_ticks,
            state: None,
            selected_pane: SelectedPane::Playlist,
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
        let playlist_list = Builder::create_playlist_list(
            &self.playlists,
            self.selected_pane == SelectedPane::Playlist,
        );
        let playlist_idx = self.playlist_state.selected().unwrap_or(0);
        let playlist_items_list = Builder::create_playlist_track_list(
            &self.playlists[playlist_idx].items,
            self.playlists[playlist_idx].name.clone(),
            self.selected_pane == SelectedPane::Tracks,
        );
        let now_playing_para = Builder::create_now_playing_para(&state);
        let time_gauge = Builder::create_progress_bar(
            state.raw_time.seconds_elapsed as f64,
            state.raw_time.track_time_seconds as f64,
        );

        let art_para = match state.album_art.clone() {
            Some(album_art) => Paragraph::new(album_art)
                .block(Block::default().borders(Borders::ALL))
                .style(Style::default()),
            None => Paragraph::new(""),
        };

        let can_update_art_width = self.redraw_ticks.can_update();
        self.terminal.draw(|frame| {
            let PlayerAreas {
                playlists,
                tracks,
                title,
                progress,
                art,
            } = Builder::create_container(frame);

            if can_update_art_width {
                self.redraw_ticks.reset();
                self.data_tx.send(SpotifyEvents::SetArtWidth(art.width as u32));
            }
            
            // new perf/ todo: only render a widget when changes
                frame.render_widget(art_para, art);
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

    // todo: should be in another struct
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
                    SpotifyEvents::PlayTrack => {
                        let playlist_idx = self.playlist_state.selected().unwrap();
                        let track_idx =
                            self.playlist_track_states[playlist_idx].selected().unwrap();

                        let playlist = &self.playlists[playlist_idx];
                        let track_id = playlist.items[track_idx].track.id.clone();

                        let playlist_id = playlist.id.clone();

                        let to_play = NewSong {
                            track_id,
                            playlist_id,
                        };

                        self.data_tx.send(SpotifyEvents::StartTrack(to_play));
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
        // hack: this whole fucking method ew
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
