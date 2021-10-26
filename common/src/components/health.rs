use serde::{Serialize, Deserialize};
use std::ops::SubAssign;
use crate::components::Damage;

#[derive(Copy, Clone, Debug, Serialize, Deserialize)]
#[cfg_attr(test, derive(PartialEq))]
pub struct Health(pub u16);

impl SubAssign<Damage> for Health {
    fn sub_assign(&mut self, damage: Damage) {
        self.0 -= damage.0;
    }
}
