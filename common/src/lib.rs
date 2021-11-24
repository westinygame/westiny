use serde::{Serialize, Deserialize};
use blaminar::simulation::laminar::LaminarConfig;
use std::time::Duration;
use std::fmt;

pub mod components;
pub mod resources;
pub mod systems;
pub mod network;
pub mod serialization;
pub mod entities;
pub mod events;
pub mod utilities;
pub mod metric_dimension;
pub mod collision;

/// The move direction relative to facing
#[derive(Copy, Clone, Debug, Serialize, Deserialize)]
pub enum MoveDirection {
    Forward,
    Backward,
    StrafeLeft,
    StrafeRight,
}

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

#[derive(Eq, PartialEq, Debug, Clone, Serialize, Deserialize)]
pub struct PlayerName(pub String);

impl fmt::Display for PlayerName {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)?;
        Ok(())
    }
}
