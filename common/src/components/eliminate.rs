use amethyst::core::Time;
use amethyst::core::ecs::{Component, VecStorage};

pub struct Eliminated {
    pub elimination_time_sec: f64,
}

impl Component for Eliminated {
    type Storage = VecStorage<Self>;
}