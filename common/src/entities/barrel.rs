use amethyst::core::ecs::{World, Entity};
use amethyst::core::math::Point2;
use amethyst::core::Transform;
use amethyst::prelude::{WorldExt, Builder};
use crate::components::BoundingCircle;

pub fn place_barrel(world: &mut World, pos: Point2<i32>) -> Entity {

    let mut transform = Transform::default();
    transform.set_translation_xyz((pos.x as f32) * 16.0, (pos.y as f32) * 16.0, 0.0);

    world
        .create_entity()
        .with(transform)
        .with(BoundingCircle{radius: 8.0})
        .build()
}
