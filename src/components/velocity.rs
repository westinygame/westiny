use amethyst::ecs::prelude::{Component, DenseVecStorage};

pub struct Velocity {
    pub velocity: f32,
}

impl Velocity {
    pub fn default() -> Velocity {
        Velocity { velocity: 0.0 }
    }
}

impl Component for Velocity {
    type Storage = DenseVecStorage<Self>;
}