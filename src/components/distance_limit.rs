
use amethyst::ecs::prelude::{Component, DenseVecStorage};

#[derive(Debug)]
pub struct DistanceLimit
{
    pub distance_to_live: f32,
}

impl Component for DistanceLimit {
    type Storage = DenseVecStorage<Self>;
}

impl DistanceLimit
{
    pub fn new(distance_to_live: f32) -> Self {
        DistanceLimit { distance_to_live }
    }
}
