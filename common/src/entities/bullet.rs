use amethyst::{
    core::transform::Transform,
    core::math::Vector2,
    ecs::prelude::Builder,
};

use crate::components::{Velocity, Projectile, Lifespan};
use std::time::Duration;
use crate::metric_dimension::{Second, MeterPerSec};

pub fn spawn_bullet<B: Builder>(
    transform: Transform,
    velocity: Vector2<MeterPerSec>,
    current_time: Duration,
    lives_for_seconds: Second,
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
