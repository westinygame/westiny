use serde::{Serialize, Deserialize};

pub mod components;
pub mod resources;
pub mod network;
pub mod serialization;

/// The move direction relative to facing
#[derive(Copy, Clone, Debug, Serialize, Deserialize)]
pub enum MoveDirection {
    Forward,
    Backward,
    StrafeLeft,
    StrafeRight,
}

pub use serialization::{serialize, deserialize};
