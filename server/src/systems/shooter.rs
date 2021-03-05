use amethyst::derive::SystemDesc;
use amethyst::ecs::{Read, System, SystemData, ReadStorage, ReadExpect, WriteExpect, Entities, WriteStorage};
use amethyst::core::{Transform, Time, math::{Vector3, Vector2}};
use amethyst::ecs::prelude::{LazyUpdate, Join};

use westiny_common::components::{weapon::Weapon, Player, Input, InputFlags, BoundingCircle};
use crate::components::EntityType;
use crate::resources::NetworkIdSupplier;
use crate::entities::spawn_bullet;

#[derive(SystemDesc)]
pub struct ShooterSystem;

/// Shooter system has some code duplication with PlayerShooterSystem.
impl<'s> System<'s> for ShooterSystem {
    type SystemData = (
        Entities<'s>,
        ReadStorage<'s, Transform>,
        ReadStorage<'s, Player>,
        ReadStorage<'s, Input>,
        ReadStorage<'s, BoundingCircle>,
        WriteStorage<'s, Weapon>,
        Read<'s, Time>,
        WriteExpect<'s, NetworkIdSupplier>,
        ReadExpect<'s, LazyUpdate>,
    );

    fn run(&mut self, (entities, transforms, players, inputs, bounds, mut weapons, time, mut net_id_supplier, lazy_update): Self::SystemData) {
        for (_player, input, player_transform, player_bound, mut weapon) in (&players, &inputs, &transforms, &bounds, &mut weapons).join() {
            if input.flags.intersects(InputFlags::SHOOT)
            {
                if weapon.is_allowed_to_shoot(time.absolute_time_seconds())
                {
                    let mut bullet_transform = Transform::default();
                    bullet_transform.set_translation(*player_transform.translation());
                    bullet_transform.set_rotation(*player_transform.rotation());

                    let direction3d = (bullet_transform.rotation() * Vector3::y()).normalize();
                    let direction2d = Vector2::new(-direction3d.x, -direction3d.y);

                    *bullet_transform.translation_mut() -= direction3d * player_bound.radius;

                    let network_id = net_id_supplier.next(EntityType::Bullet);
                    spawn_bullet(network_id, bullet_transform, direction2d, &weapon.details, &entities, &lazy_update);
                    weapon.last_shot_time = time.absolute_time_seconds();
                    weapon.input_lifted = false;
                }
            }
            else
            {
                weapon.input_lifted = true;
            }
        }
    }
}
