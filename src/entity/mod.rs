use std::f32::consts::PI;

pub mod player;

#[derive(Copy, Clone)]
pub enum Direction {
    FORWARD,
    BACKWARD,
    LEFT,
    RIGHT,
}

impl Direction {
    /// Returns the mapped rotation in radians. e.g. FORWARD means no need for rotation
    pub fn rotation(&self) -> f32 {
        match self {
            Direction::FORWARD => 0.0,
            Direction::LEFT => PI / 2.0,
            Direction::BACKWARD => PI,
            Direction::RIGHT => -PI / 2.0
        }
    }
}