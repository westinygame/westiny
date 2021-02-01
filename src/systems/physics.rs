use amethyst::derive::SystemDesc;
use amethyst::ecs::{Read, System, SystemData, ReadStorage, WriteStorage, ReadExpect, Entities};
use amethyst::ecs::prelude::Join;
use amethyst::core::{Transform, Time};
use amethyst::core::math::Vector2;

use crate::components::{Velocity, DistanceLimit};
use crate::resources::Sounds;
use amethyst::assets::AssetStorage;
use amethyst::audio::{Source, output::Output};

#[derive(SystemDesc)]
pub struct PhysicsSystem;

impl<'s> System<'s> for PhysicsSystem {
    type SystemData = (
        Entities<'s>,
        WriteStorage<'s, Transform>,
        ReadStorage<'s, Velocity>,
        Read<'s, Time>,
        WriteStorage<'s, DistanceLimit>,
        Read<'s, AssetStorage<Source>>,
        ReadExpect<'s, Sounds>,
        Read<'s, Output>
    );

    fn run(&mut self, (entities, mut transforms, velocities, time, mut distance_limits, audio_storage, sounds, sound_output): Self::SystemData) {
        for (moving_entity, transform, velocity, maybe_distance_limit) in
            (&*entities, &mut transforms, &velocities, (&mut distance_limits).maybe()).join()
        {
            let delta_s = update_position(transform, velocity, &time).norm();

            if let Some(distance_limit) = maybe_distance_limit {
                distance_limit.distance_to_live -= delta_s;
                if distance_limit.distance_to_live < 0.0 {
                    entities.delete(moving_entity).expect("Could not delete distance limited entity!");
                    if let Some(sound) = audio_storage.get(&sounds.dirt_hit) {
                        (*sound_output).play_once(sound, 1.0);
                    }
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
