use crate::types::app::event_types::NewSong;

pub enum SpotifyEvents {
    NavigateUp,
    NavigateDown,
    NavigateLeft,
    NavigateRight,
    PlayTrack,
    StartTrack(NewSong), // string === track id
    Quit,
    NewArt,
    SetArtWidth(u32),
}
