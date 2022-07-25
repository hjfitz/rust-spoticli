use crate::types::full::playlist::Item;

pub struct AppPlaylist {
    pub id: String,
    pub name: String,
    pub items: Vec<Item>,
}
