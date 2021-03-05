use std::time::Duration;
use amethyst::core::ecs::{Component, VecStorage};

/// This component marks if an entity should respawn after being eliminated
#[derive(Copy, Clone, Debug)]
pub struct Respawn {
    pub respawn_duration: Duration,
}

impl Component for Respawn {
    type Storage = VecStorage<Self>;
}
