use serde::{Serialize, Deserialize};

pub mod components;
pub mod resources;
pub mod network;

/// The move direction relative to facing
#[derive(Copy, Clone, Debug, Serialize, Deserialize)]
pub enum MoveDirection {
    Forward,
    Backward,
    StrafeLeft,
    StrafeRight,
}
