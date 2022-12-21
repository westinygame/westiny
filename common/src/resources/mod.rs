pub use audio::{AudioQueue, SoundId};
pub use map::build_map;

mod audio;
pub mod collision;
pub mod map;
pub mod weapon;

use serde::{Deserialize, Serialize};
use std::fmt::{Display, Formatter};
use std::net::SocketAddr;
use bevy::prelude::Resource;

const DEFAULT_SERVER_PORT: u16 = 5745;

#[derive(Clone, Deserialize, Eq, PartialEq, Resource)]
pub struct ServerAddress {
    pub address: SocketAddr,
}

impl Default for ServerAddress {
    fn default() -> Self {
        ServerAddress {
            address: SocketAddr::new("127.0.0.1".parse().unwrap(), DEFAULT_SERVER_PORT),
        }
    }
}

#[derive(Copy, Clone, Serialize, Deserialize, Eq, PartialEq, Debug, Hash, Default, Resource)]
pub struct Seed(pub u64);

impl Display for Seed {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        self.0.fmt(f)
    }
}

#[derive(Resource)]
pub struct ResourcesDir {
    pub common_resources: std::path::PathBuf,
    pub crate_resources: std::path::PathBuf,
}
