use amethyst::derive::SystemDesc;
use amethyst::ecs::{Read, System, SystemData, ReadStorage, WriteStorage};
use amethyst::ecs::prelude::Join;
use amethyst::core::{Transform, Time};
use amethyst::core::math::Vector2;

use crate::components::{Velocity};
use crate::metric_dimension::Second;
use crate::metric_dimension::length::Meter;

#[derive(SystemDesc)]
pub struct PhysicsSystem;

impl<'s> System<'s> for PhysicsSystem {
    type SystemData = (
        WriteStorage<'s, Transform>,
        ReadStorage<'s, Velocity>,
        Read<'s, Time>,
    );

    fn run(&mut self, (mut transforms, velocities, time): Self::SystemData) {
        for (transform, velocity) in
            (&mut transforms, &velocities).join()
        {
            update_position(transform, velocity, &time);
        }
    }
}

/// Updates transform with velocity based on time
/// Returns delta (x,y) vector
pub fn update_position(transform: &mut Transform, velocity: &Velocity, time: &Time) -> Vector2<Meter> {
    let delta = Second(time.delta_seconds()) * velocity.0;
    let delta_clone = delta.clone();
    transform.prepend_translation_x(delta_clone.x.into_pixel());
    transform.prepend_translation_y(delta_clone.y.into_pixel());
    delta
}

#[cfg(test)]
mod test {
    use super::*;
    use amethyst::core::math::Vector2;
    use crate::metric_dimension::MeterPerSec;

    #[test]
    fn test_update_position() {
        let mut transform = Transform::default();
        transform.set_translation_x(Meter(100.0).into_pixel());
        transform.set_translation_y(Meter(100.0).into_pixel());

        let velocity = Velocity(Vector2::new(MeterPerSec(-50.0), MeterPerSec(-50.0)));
        let mut time = Time::default();
        time.set_delta_seconds(0.5);

        update_position(&mut transform, &velocity, &time);

        assert_eq!(transform.translation().x.round(), Meter(75.0).into_pixel());
        assert_eq!(transform.translation().y.round(), Meter(75.0).into_pixel());
    }
}
