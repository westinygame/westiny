pub use audio::{SoundId, AudioQueue};
pub use entity_delete::EntityDelete;

mod audio;
mod entity_delete;
pub mod map;

use std::fmt::{Display, Formatter};
use std::net::SocketAddr;
use serde::{Serialize, Deserialize};

const DEFAULT_SERVER_PORT: u16 = 5745;

#[derive(Clone, Deserialize)]
pub struct ServerAddress {
    pub address: SocketAddr,
}

impl Default for ServerAddress {
    fn default() -> Self {
        ServerAddress { address: SocketAddr::new("127.0.0.1".parse().unwrap(), DEFAULT_SERVER_PORT)}
    }
}

#[derive(Copy, Clone)]
#[repr(usize)]
pub enum SpriteId {
    Player = 3,
    #[allow(dead_code)] // Please remove this allow when using ShootingPlayer
    ShootingPlayer = 4,
    Bullet = 5,
    Barrel = 6,
}

#[derive(Copy, Clone, Serialize, Deserialize, PartialEq, Debug)]
pub struct Seed(pub u64);

impl Display for Seed {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        self.0.fmt(f)
    }
}