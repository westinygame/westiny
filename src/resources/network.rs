use std::net::SocketAddr;
use serde::Deserialize;

const DEFAULT_SERVER_PORT: u16 = 5745;
const DEFAULT_CLIENT_PORT: u16 = 4557;

#[derive(Clone, Deserialize)]
pub struct ServerAddress {
    pub address: SocketAddr,
}

impl Default for ServerAddress {
    fn default() -> Self {
        ServerAddress { address: SocketAddr::new("127.0.0.1".parse().unwrap(), DEFAULT_SERVER_PORT)}
    }
}

#[derive(Deserialize)]
pub struct ClientPort(pub u16);

impl Default for ClientPort {
    fn default() -> Self {
        ClientPort(DEFAULT_CLIENT_PORT)
    }
}
