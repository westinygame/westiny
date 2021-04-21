use amethyst::core::ecs::{World, Entity};
use amethyst::core::math::Point2;
use amethyst::core::Transform;
use amethyst::prelude::{WorldExt, Builder};
use crate::components::BoundingCircle;
use crate::metric_dimension::length::Meter;

const BARREL_HEIGHT: f32 = 1.0;
const BARREL_DIAMETER: Meter = Meter(1.0);

pub fn place_barrel(world: &mut World, pos: Point2<i32>) -> Entity {

    let mut transform = Transform::default();
    transform.set_translation_xyz((pos.x as f32) * BARREL_DIAMETER.into_pixel(), (pos.y as f32) * BARREL_DIAMETER.into_pixel(), BARREL_HEIGHT);

    world
        .create_entity()
        .with(transform)
        .with(BoundingCircle{radius: Meter(0.5)})
        .build()
}
