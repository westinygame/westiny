pub use barrel::{place_barrel, BarrelBundle};
pub use bullet::spawn_bullet;

mod barrel;
mod bullet;

use crate::components::SpriteId;
use bevy::prelude::{Bundle, GlobalTransform, Transform};

#[derive(Bundle)]
pub struct SimpleSpriteSheetBundle {
    pub global_transform: GlobalTransform,
    pub transform: Transform,
    pub sprite: SpriteId,
}

impl SimpleSpriteSheetBundle {
    pub fn new(transform: Transform, sprite: SpriteId) -> Self {
        SimpleSpriteSheetBundle {
            global_transform: GlobalTransform::default(),
            transform,
            sprite,
        }
    }
}
