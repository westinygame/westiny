use crate::components::BoundingCircle;
use crate::metric_dimension::length::Meter;
use crate::resources::SpriteId;
use bevy::prelude::{Commands, Transform, GlobalTransform, Vec2};

const BARREL_HEIGHT: f32 = 1.0;
const BARREL_DIAMETER: Meter = Meter(1.0);

pub fn place_barrel(commands: &mut Commands, pos: Vec2) {
    let transform = Transform::from_xyz(pos.x * BARREL_DIAMETER.into_pixel(),
                                                      pos.y * BARREL_DIAMETER.into_pixel(),
                                                      BARREL_HEIGHT);

    commands.spawn()
        .insert(transform)
        .insert(GlobalTransform::identity())
        .insert(BoundingCircle { radius: BARREL_DIAMETER / 2f32})
        .insert(SpriteId::Barrel);

}
