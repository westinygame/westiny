pub use barrel::{place_barrel, BarrelBundle};
pub use bullet::BulletBundle;

mod barrel;
mod bullet;

use crate::components::SpriteId;
use bevy::prelude::{Bundle, SpatialBundle, Transform};

#[derive(Bundle)]
pub struct SimpleSpriteSheetBundle {
    pub sprite: SpriteId,

    #[bundle]
    pub spatial: SpatialBundle
}

impl SimpleSpriteSheetBundle {
    pub fn new(transform: Transform, sprite: SpriteId) -> Self {
        SimpleSpriteSheetBundle {
            sprite,
            spatial: SpatialBundle::from_transform(transform)
        }
    }
}
