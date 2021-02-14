use amethyst::core::ecs::{System, WriteStorage, ReadExpect};
use crate::components::Health;
use crate::resources::ProjectileCollisions;
use log::logger;

pub struct DamageSystem;

impl <'s> System<'s> for DamageSystem {
    type SystemData = (
        WriteStorage<'s, Health>,
        ReadExpect<'s, ProjectileCollisions>
    );

    fn run(&mut self, (mut healths, projectileCollisions): Self::SystemData) {
        for collision in &projectileCollisions.0 {
            if let Some(health) = healths.get_mut(collision.target) {
                health.0 -= 5;
                log::info!("Hit")
            }
        }
    }
}