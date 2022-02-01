use crate::components::{Damage, Lifespan, Projectile, Velocity};
use crate::metric_dimension::{MeterPerSecVec2, Second};
use bevy::prelude::{Commands, Transform};
use std::time::Duration;

pub fn spawn_bullet(
    commands: &mut Commands,
    damage: u16,
    transform: Transform,
    velocity: MeterPerSecVec2,
    current_time: Duration,
    lives_for_seconds: Second,
) {
    let lifespan = Lifespan::new(lives_for_seconds, current_time);

    commands.spawn_bundle((
        transform,
        Velocity(velocity),
        Projectile {},
        lifespan,
        Damage(damage),
    ));
}
