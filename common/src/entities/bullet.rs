use amethyst::{
    core::transform::Transform,
    core::math::Vector2,
    ecs::prelude::Builder,
};

use crate::components::{Velocity, Projectile, Lifespan};
use std::time::Duration;

pub fn spawn_bullet<B: Builder>(
    transform: Transform,
    velocity: Vector2<f32>,
    current_time: Duration,
    lives_for_seconds: f32,
    entity_builder: B)
{
    let lifespan = Lifespan::new(lives_for_seconds, current_time);
    let velocity = Velocity(velocity);

    entity_builder
        .with(transform)
        .with(velocity)
        .with(Projectile::default())
        .with(lifespan)
        .build();
}
