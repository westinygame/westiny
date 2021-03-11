use amethyst::{
    core::transform::Transform,
    core::math::Vector2,
    ecs::prelude::{Builder, Entities, LazyUpdate, ReadExpect},
};

use westiny_common::components::{Velocity, weapon::WeaponDetails, Projectile, TimeLimit, NetworkId, Damage};
use std::time::Duration;

pub fn spawn_bullet(network_id: NetworkId,
                    transform: Transform,
                    direction: Vector2<f32>,
                    current_time: Duration,
                    weapon_details: &WeaponDetails, 
                    entities: &Entities,
                    lazy_update: &ReadExpect<LazyUpdate>)
{
    let time_limit = TimeLimit::new(weapon_details.bullet_time_limit, current_time);
    let velocity = Velocity(direction * weapon_details.bullet_speed);

    lazy_update
        .create_entity(entities)
        .with(network_id)
        .with(transform)
        .with(velocity)
        .with(Projectile::default())
        .with(time_limit)
        .with(Damage(weapon_details.damage))
        .build();
}
