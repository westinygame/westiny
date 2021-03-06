use amethyst::derive::SystemDesc;
use amethyst::ecs::{Read, System, SystemData, ReadStorage, WriteStorage};
use amethyst::ecs::prelude::Join;
use amethyst::core::{Transform, Time};
use amethyst::core::math::Vector2;

use crate::components::{Velocity};

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
            update_position(transform, velocity, &time).norm();
        }
    }
}

/// Updates transform with velocity based on time
/// Returns delta (x,y) vector
pub fn update_position(transform: &mut Transform, velocity: &Velocity, time: &Time) -> Vector2<f32> {
    let delta = velocity.0 * time.delta_seconds();
    transform.prepend_translation_x(delta.x);
    transform.prepend_translation_y(delta.y);
    delta
}

#[cfg(test)]
mod test {
    use super::*;
    use amethyst::core::math::Vector2;

    #[test]
    fn test_update_position() {
        let mut transform = Transform::default();
        transform.set_translation_x(100.0);
        transform.set_translation_y(100.0);

        let velocity = Velocity(Vector2::new(-50.0, -50.0));
        let mut time = Time::default();
        time.set_delta_seconds(0.5);

        update_position(&mut transform, &velocity, &time);

        assert_eq!(transform.translation().x.round(), 75.0);
        assert_eq!(transform.translation().y.round(), 75.0);
    }
}
