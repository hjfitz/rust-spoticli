pub trait Updatable {
    fn can_update(&self, time: u32) -> bool;
}
