
use amethyst::{
    core::transform::Transform,
    core::math::Vector2,
    ecs::prelude::{Builder, Entities, LazyUpdate, ReadExpect},
    renderer::SpriteRender
};

use westiny_common::components::{Velocity, weapon::WeaponDetails, Projectile, DistanceLimit};

pub fn spawn_bullet(transform: Transform, direction: Vector2<f32>, weapon_details: &WeaponDetails, sprite_render: SpriteRender,
                    entities: &Entities, lazy_update: &ReadExpect<LazyUpdate>)
{
    let distance_limit = DistanceLimit::new(weapon_details.distance);
    let velocity = Velocity(direction * weapon_details.bullet_speed);

    lazy_update
        .create_entity(entities)
        .with(transform)
        .with(velocity)
        .with(Projectile::default())
        .with(sprite_render)
        .with(distance_limit)
        .build();
}
