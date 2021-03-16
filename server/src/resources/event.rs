use super::ClientID;

use westiny_common::components::Input;
use westiny_common::PlayerName;

#[derive(Debug, Eq, PartialEq)]
pub enum ClientNetworkEvent {
    ClientConnected(ClientID),
    ClientDisconnected(ClientID, PlayerName),
}

#[derive(Debug, PartialEq)]
pub enum NetworkCommand {
    Input {
        id: ClientID,
        input: Input
    }
}
