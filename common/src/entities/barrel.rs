use crate::components::{BoundingCircle, SpriteId};
use crate::metric_dimension::length::Meter;
use bevy::prelude::{Bundle, Commands, Transform, Vec2};

const BARREL_HEIGHT: f32 = 1.0;
const BARREL_DIAMETER: Meter = Meter(1.0);

pub fn place_barrel(commands: &mut Commands, pos: Vec2) {
    let transform = Transform::from_xyz(
        pos.x * BARREL_DIAMETER.into_pixel(),
        pos.y * BARREL_DIAMETER.into_pixel(),
        BARREL_HEIGHT,
    );

    commands.spawn(BarrelBundle {
        bounding_circle: BoundingCircle {
            radius: BARREL_DIAMETER / 2f32,
        },
        sprite_sheet_bundle: super::SimpleSpriteSheetBundle::new(transform, SpriteId::Barrel),
    });
}

#[derive(Bundle)]
pub struct BarrelBundle {
    bounding_circle: BoundingCircle,
    #[bundle]
    sprite_sheet_bundle: super::SimpleSpriteSheetBundle,
}
