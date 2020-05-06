use amethyst::ecs::prelude::{Component, DenseVecStorage};

pub struct BoundingCircle {
    pub radius: f32,
}

impl Component for BoundingCircle {
    type Storage = DenseVecStorage<Self>;
}

