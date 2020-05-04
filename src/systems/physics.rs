use amethyst::derive::SystemDesc;
use amethyst::ecs::{Read, System, SystemData, ReadStorage, WriteStorage};
use amethyst::ecs::prelude::Join;
use amethyst::core::{Transform, Time};

use crate::components::Velocity;

#[derive(SystemDesc)]
pub struct PhysicsSystem;

impl<'s> System<'s> for PhysicsSystem {
    type SystemData = (
        WriteStorage<'s, Transform>,
        ReadStorage<'s, Velocity>,
        Read<'s, Time>
    );

    fn run(&mut self, (mut transforms, velocities, time): Self::SystemData) {
        for (transform, velocity) in (&mut transforms, &velocities).join() {
            update_position(transform, velocity, &time);
        }
    }
}

fn update_position(transform: &mut Transform, velocity: &Velocity, time: &Time) {
    transform.prepend_translation_x(velocity.0.x * time.delta_seconds());
    transform.prepend_translation_y(velocity.0.y * time.delta_seconds());
}
