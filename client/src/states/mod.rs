pub mod connection;
pub mod play;

#[derive(Debug, Clone, Eq, PartialEq, Hash)]
pub enum AppState {
    Connect,
    PlayInit,
    Play,
}
