use super::ClientID;

use westiny_common::components::Input;
use westiny_common::PlayerName;

#[derive(Debug, Eq, PartialEq)]
#[cfg_attr(test, derive(bevy::prelude::Resource))]
pub enum ClientNetworkEvent {
    ClientConnected(ClientID),
    ClientDisconnected(ClientID, PlayerName),
}

#[derive(Debug, PartialEq)]
#[cfg_attr(test, derive(bevy::prelude::Resource))]
pub enum NetworkCommand {
    Input { id: ClientID, input: Input },
}
