use super::ClientID;

#[derive(Debug, Eq, PartialEq)]
pub enum ClientNetworkEvent {
    ClientConnected(ClientID),
    ClientDisconnected(ClientID),
}