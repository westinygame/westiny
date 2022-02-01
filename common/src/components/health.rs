use crate::components::Damage;
use bevy::ecs::component::Component;
use serde::{Deserialize, Serialize};
use std::ops::SubAssign;

#[derive(Copy, Clone, Debug, Serialize, Deserialize, Component)]
#[cfg_attr(test, derive(PartialEq))]
pub struct Health(pub u16);

impl SubAssign<Damage> for Health {
    fn sub_assign(&mut self, damage: Damage) {
        self.0 -= damage.0;
    }
}
