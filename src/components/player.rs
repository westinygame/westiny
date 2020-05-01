
use amethyst::ecs::prelude::{Component, DenseVecStorage};

pub struct Player;

impl Component for Player {
    type Storage = DenseVecStorage<Self>;
}

