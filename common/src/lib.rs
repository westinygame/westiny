use serde::{Serialize, Deserialize};
use amethyst::network::simulation::laminar::LaminarConfig;
use std::time::Duration;

pub mod components;
pub mod resources;
pub mod systems;
pub mod network;
pub mod serialization;
pub mod entities;
pub mod collision;
pub mod events;
pub mod utilities;

/// The move direction relative to facing
#[derive(Copy, Clone, Debug, Serialize, Deserialize)]
pub enum MoveDirection {
    Forward,
    Backward,
    StrafeLeft,
    StrafeRight,
}

pub use serialization::{serialize, deserialize};


#[derive(Deserialize)]
pub struct NetworkConfig {
    hartbeat_interval: u8,
}

impl Into<LaminarConfig> for NetworkConfig {
    fn into(self) -> LaminarConfig {
        let mut laminar = LaminarConfig::default();
        laminar.heartbeat_interval = Some(Duration::from_secs(self.hartbeat_interval as u64));
        laminar
    }
}
