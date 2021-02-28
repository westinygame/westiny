use amethyst::ecs::prelude::{Component, VecStorage};
use serde::{Serialize, Deserialize};
use std::ops::SubAssign;
use crate::components::Damage;

#[derive(Copy, Clone, Debug, Serialize, Deserialize)]
pub struct Health(pub u16);

impl Component for Health {
    type Storage = VecStorage<Self>;
}

impl SubAssign<Damage> for Health {
    fn sub_assign(&mut self, damage: Damage) {
        self.0 -= damage.0;
    }
}
