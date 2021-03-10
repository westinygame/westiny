use amethyst::derive::SystemDesc;
use amethyst::ecs::{Read, Write, System, SystemData, ReadStorage, WriteStorage, Entities};
use amethyst::shrev::EventChannel;
use amethyst::ecs::prelude::Join;
use amethyst::core::{Transform, Time};
use amethyst::core::math::Vector2;

use crate::components::{Velocity, DistanceLimit};
use crate::resources::EntityDelete;

#[derive(SystemDesc)]
pub struct PhysicsSystem;

impl<'s> System<'s> for PhysicsSystem {
    type SystemData = (
        Entities<'s>,
        WriteStorage<'s, Transform>,
        ReadStorage<'s, Velocity>,
        Read<'s, Time>,
        WriteStorage<'s, DistanceLimit>,
        Write<'s, EventChannel<EntityDelete>>
    );

    fn run(&mut self, (entities, mut transforms, velocities, time, mut distance_limits, mut delete_entity_channel): Self::SystemData) {
        for (moving_entity, transform, velocity, maybe_distance_limit) in
            (&*entities, &mut transforms, &velocities, (&mut distance_limits).maybe()).join()
        {
            let delta_s = update_position(transform, velocity, &time).norm();

            if let Some(distance_limit) = maybe_distance_limit {
                distance_limit.distance_to_live -= delta_s;
                if distance_limit.distance_to_live < 0.0 {
                    delete_entity_channel.single_write(EntityDelete{entity_id: moving_entity})
                }
            }
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
