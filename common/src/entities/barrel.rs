use crate::components::BoundingCircle;
use crate::metric_dimension::length::Meter;
use bevy::ecs::prelude::Commands;
use bevy::prelude::{Transform, Vec2};

const BARREL_HEIGHT: f32 = 1.0;
const BARREL_DIAMETER: Meter = Meter(0.0);

pub fn place_barrel(commands: &mut Commands, pos: Vec2) {
    let mut transform = Transform::from_xyz(pos.x * BARREL_DIAMETER.into_pixel(),
                                                      pos.y * BARREL_DIAMETER.into_pixel(),
                                                      BARREL_HEIGHT);

    commands.spawn()
        .insert(transform)
        .insert(BoundingCircle { radius: BARREL_DIAMETER / 2f32});
}
