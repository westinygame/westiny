use amethyst::ecs::prelude::{Component, VecStorage};

#[derive(Debug)]
pub struct Health(pub u16);

impl Component for Health {
    type Storage = VecStorage<Self>;
}
