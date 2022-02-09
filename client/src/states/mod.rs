use westiny_common::network::ClientInitialData;

pub mod connection;
//pub mod game_states;

#[derive(Debug, Clone, Eq, PartialEq, Hash)]
pub enum AppState {
    Connect,
    InGame(ClientInitialData),
}
