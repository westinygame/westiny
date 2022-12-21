pub mod connection;
pub mod play;

#[derive(Debug, Clone, Eq, PartialEq, Hash)]
#[cfg_attr(test, derive(bevy::prelude::Resource))]
pub enum AppState {
    Connect,
    PlayInit,
    Play,
}
