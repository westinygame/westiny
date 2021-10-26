use crate::components::{Velocity, Projectile, Lifespan};
use std::time::Duration;
use crate::metric_dimension::{Second, MeterPerSecVec2};
use bevy::prelude::{Transform, Commands};

pub fn spawn_bullet(commands: &mut Commands,
                    transform: Transform,
                    velocity: MeterPerSecVec2,
                    current_time: Duration,
                    lives_for_seconds: Second)
{

    let lifespan = Lifespan::new(lives_for_seconds, current_time);

    commands.spawn_bundle((
        transform,
        Velocity(velocity),
        Projectile {},
        lifespan));
}
