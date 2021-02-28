use amethyst::core::ecs::{Component, VecStorage};

#[derive(Copy, Clone, Debug)]
pub struct Damage(pub u16);

impl Component for Damage {
    type Storage = VecStorage<Self>;
}