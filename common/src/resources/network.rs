use std::net::SocketAddr;
use serde::Deserialize;
use crate::components;

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

#[derive(Default)]
pub struct NetworkIdSupplier {
    next_id: std::cell::RefCell<u64>,
}

impl NetworkIdSupplier {
    pub fn new() -> Self {
        NetworkIdSupplier { next_id: std::cell::RefCell::new(0) }
    }

    pub fn next(&self) -> components::NetworkId {
        components::NetworkId::new(self.next_id.replace_with(|current| *current + 1))
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn network_id_supplier_get_and_increment() {
        let supplier = NetworkIdSupplier::new();
        for i in 0..100_000 {
            assert_eq!(&i, supplier.next().get());
        }
    }
}
