use amethyst::derive::SystemDesc;
use amethyst::ecs::{Read, System, SystemData, ReadStorage, ReadExpect, Entities, WriteStorage, WriteExpect};
use amethyst::core::{Transform, Time, math::{Vector3, Vector2}};
use amethyst::ecs::prelude::{LazyUpdate, Join};

use westiny_common::components::{weapon::Weapon, Player, Input, InputFlags, BoundingCircle};
use crate::components::Damage;
use westiny_common::entities::spawn_bullet;
use amethyst::prelude::Builder;
use crate::resources::{ClientRegistry, StreamId};
use amethyst::network::simulation::{TransportResource, DeliveryRequirement, UrgencyRequirement};
use westiny_common::serialize;
use westiny_common::network::{PacketType, ShotEvent};
use amethyst::core::math::Point2;

#[derive(SystemDesc)]
pub struct ShooterSystem;

impl<'s> System<'s> for ShooterSystem {
    type SystemData = (
        Entities<'s>,
        ReadStorage<'s, Transform>,
        ReadStorage<'s, Player>,
        ReadStorage<'s, Input>,
        ReadStorage<'s, BoundingCircle>,
        WriteStorage<'s, Weapon>,
        Read<'s, Time>,
        ReadExpect<'s, LazyUpdate>,
        ReadExpect<'s, ClientRegistry>,
        WriteExpect<'s, TransportResource>
    );

    fn run(&mut self, (entities, transforms, players, inputs, bounds, mut weapons, time, lazy_update, client_registry, mut net): Self::SystemData) {
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

                    let velocity = direction2d * weapon.details.bullet_speed;
                    let bullet_builder = lazy_update.create_entity(&entities)
                        .with(Damage(weapon.details.damage));

                    spawn_bullet(bullet_transform.clone(),
                                 velocity.clone(),
                                 time.absolute_time(),
                                 weapon.details.bullet_time_limit,
                                 bullet_builder);

                    weapon.last_shot_time = time.absolute_time_seconds();
                    weapon.input_lifted = false;

                    let payload = serialize(&PacketType::ShotEvent(ShotEvent {
                        position: Point2::new(bullet_transform.translation().x, bullet_transform.translation().y),
                        velocity,
                        bullet_time_limit_secs: weapon.details.bullet_time_limit,
                    })).expect("ShotEvent's serialization failed");

                    client_registry.get_clients().iter().map(|handle| handle.addr).for_each(|addr| {
                        net.send_with_requirements(addr,
                                                   &payload,
                                                   DeliveryRequirement::ReliableSequenced(StreamId::ShotEvent.into()),
                                                   UrgencyRequirement::OnTick);
                    })
                }
            }
            else
            {
                weapon.input_lifted = true;
            }
        }
    }
}
