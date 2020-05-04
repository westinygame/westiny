use amethyst::ecs::prelude::{Component, DenseVecStorage};
use amethyst::core::math::Vector2;

#[derive(Debug)]
pub struct Velocity(pub Vector2<f32>);

impl Component for Velocity {
    type Storage = DenseVecStorage<Self>;
}

impl Default for Velocity {
    fn default() -> Self {
        Velocity(Vector2::new(0.0, 0.0))
    }
}