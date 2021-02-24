use serde::{Serialize, Deserialize};

pub mod components;
pub mod resources;
pub mod systems;
pub mod network;
pub mod serialization;
pub mod entities;
pub mod collision;

/// The move direction relative to facing
#[derive(Copy, Clone, Debug, Serialize, Deserialize)]
pub enum MoveDirection {
    Forward,
    Backward,
    StrafeLeft,
    StrafeRight,
}

pub use serialization::{serialize, deserialize};
