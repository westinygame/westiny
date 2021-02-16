use amethyst::{
    core::transform::Transform,
    core::math::Vector2,
    ecs::prelude::{Builder, Entities, LazyUpdate, ReadExpect},
};

use westiny_common::components::{Velocity, weapon::WeaponDetails, Projectile, DistanceLimit, NetworkId};

pub fn spawn_bullet(network_id: NetworkId,
                    transform: Transform,
                    direction: Vector2<f32>,
                    weapon_details: &WeaponDetails, 
                    entities: &Entities,
                    lazy_update: &ReadExpect<LazyUpdate>)
{
    let distance_limit = DistanceLimit::new(weapon_details.distance);
    let velocity = Velocity(direction * weapon_details.bullet_speed);

    lazy_update
        .create_entity(entities)
        .with(network_id)
        .with(transform)
        .with(velocity)
        .with(Projectile::default())
        .with(distance_limit)
        .build();
}
