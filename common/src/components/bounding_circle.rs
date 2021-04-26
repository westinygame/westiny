use amethyst::ecs::prelude::{Component, DenseVecStorage};
use crate::metric_dimension::length::Meter;

#[derive(Debug)]
pub struct BoundingCircle {
    pub radius: Meter,
}

impl Component for BoundingCircle {
    type Storage = DenseVecStorage<Self>;
}

