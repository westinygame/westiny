
use amethyst::ecs::prelude::{Component, DenseVecStorage};
use std::time::Duration;

#[derive(Debug)]
pub struct Lifespan
{
    pub living_until: Duration,
}

impl Component for Lifespan {
    type Storage = DenseVecStorage<Self>;
}

impl Lifespan
{
    pub fn new(secs_to_live: f32, timing_start: Duration) -> Self {
        Lifespan {
            living_until: timing_start + Duration::from_secs_f32(secs_to_live),
        }
    }
}
