use amethyst::{
    core::transform::Transform,
    core::math::Vector2,
    ecs::prelude::Builder,
};

use crate::components::{Velocity, Projectile, TimeLimit};
use std::time::Duration;

pub fn spawn_bullet<B: Builder>(
    transform: Transform,
    velocity: Vector2<f32>,
    current_time: Duration,
    time_limit: f32,
    entity_builder: B)
{
    let time_limit = TimeLimit::new(time_limit, current_time);
    let velocity = Velocity(velocity);

    entity_builder
        .with(transform)
        .with(velocity)
        .with(Projectile::default())
        .with(time_limit)
        .build();
}
