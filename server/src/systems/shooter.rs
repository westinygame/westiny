use amethyst::ecs::{Read, System, ReadStorage, ReadExpect, Entities, WriteStorage, WriteExpect};
use amethyst::core::{Transform, Time, math::{Vector3, Vector2}};
use amethyst::ecs::prelude::{LazyUpdate, Join};

use westiny_common::components::{weapon::Weapon, Input, InputFlags, BoundingCircle};
use crate::components::{Damage, Client, Holster};
use westiny_common::entities::spawn_bullet;
use amethyst::prelude::Builder;
use crate::resources::{ClientRegistry, StreamId, ClientID};
use amethyst::network::simulation::{TransportResource, DeliveryRequirement, UrgencyRequirement};
use westiny_common::serialize;
use westiny_common::network::{PacketType, ShotEvent, PlayerUpdate};
use amethyst::core::math::Point2;
use std::time::Duration;

pub struct ShooterSystem;

impl<'s> System<'s> for ShooterSystem {
    type SystemData = (
        Entities<'s>,
        ReadStorage<'s, Transform>,
        ReadStorage<'s, Input>,
        ReadStorage<'s, BoundingCircle>,
        WriteStorage<'s, Weapon>,
        WriteStorage<'s, Holster>,
        ReadStorage<'s, Client>,
        Read<'s, Time>,
        ReadExpect<'s, LazyUpdate>,
        ReadExpect<'s, ClientRegistry>,
        WriteExpect<'s, TransportResource>
    );

    fn run(&mut self, (entities, transforms, inputs, bounds, mut weapons, mut holsters, clients, time, lazy_update, client_registry, mut net): Self::SystemData) {
        for (input, player_transform, bound, mut weapon, holster, client) in (&inputs, &transforms, (&bounds).maybe(), &mut weapons, &mut holsters, (&clients).maybe()).join() {

            if let Some(selected_slot) = Self::selected_slot(&input) {
                if holster.active_slot() != selected_slot {
                    if let Some(weapon_details) = holster.switch(selected_slot) {
                        weapon.details = weapon_details.clone();
                        // TODO Send PlayerUpdate on weapon switch
                    }
                }
            }

            if input.flags.intersects(InputFlags::SHOOT) {
                if weapon.is_allowed_to_shoot(time.absolute_time_seconds()) {
                    Self::shoot(&entities, &time, &lazy_update, &client_registry, &mut net, player_transform, bound, &mut weapon, client);
                }
            } else {
                weapon.input_lifted = true;
            }

            if let Some(reload_start) = weapon.reload_started_at {
                Self::check_reload_finish(&time, &client_registry, &mut net, weapon, client, &reload_start)
            }
        }
    }
}


impl ShooterSystem {
    fn send_ammo_update(
        client_id: &ClientID,
        client_registry: &ClientRegistry,
        ammo_in_magazine: u32,
        net: &mut TransportResource,
    ) -> anyhow::Result<()> {
        let payload = serialize(&PacketType::PlayerUpdate(PlayerUpdate::AmmoUpdate{ammo_in_magazine}))
            .map_err(|err| anyhow::anyhow!("Failed to serialize AmmoUpdate: {}", err))?;
        let address = client_registry.find_client(*client_id).map(|handle| handle.addr)
            .ok_or(anyhow::anyhow!("Client with id {:?} not found in registry", client_id))?;
        net.send_with_requirements(address,
                                   &payload,
                                   DeliveryRequirement::ReliableSequenced(StreamId::PlayerUpdate.into()),
                                   UrgencyRequirement::OnTick
        );
        Ok(())
    }

    fn shoot(entities: &Entities,
             time: &Time,
             lazy_update: &LazyUpdate,
             client_registry: &ClientRegistry,
             mut net: &mut TransportResource,
             player_transform: &Transform,
             bound: Option<&BoundingCircle>,
             mut weapon: &mut Weapon,
             client: Option<&Client>) {
        let mut bullet_transform = Transform::default();
        bullet_transform.set_translation(*player_transform.translation());
        bullet_transform.set_rotation(*player_transform.rotation());

        let direction3d = (bullet_transform.rotation() * Vector3::y()).normalize();
        let direction2d = Vector2::new(-direction3d.x, -direction3d.y);

        if let Some(bound) = bound {
            *bullet_transform.translation_mut() -= direction3d * bound.radius;
        }

        let velocity = direction2d * weapon.details.bullet_speed;
        let bullet_builder = lazy_update.create_entity(&entities)
            .with(Damage(weapon.details.damage));

        spawn_bullet(bullet_transform.clone(),
                     velocity.clone(),
                     time.absolute_time(),
                     weapon.bullet_lifespan_sec(),
                     bullet_builder);

        weapon.last_shot_time = time.absolute_time_seconds();
        weapon.input_lifted = false;
        weapon.bullets_left_in_magazine -= 1;

        if let Some(client) = client {
            if let Err(err) = Self::send_ammo_update(&client.id,
                                                     &client_registry,
                                                     weapon.bullets_left_in_magazine,
                                                     &mut net) {
                log::error!("Failed to send ammo update to client {:?}. Error: {}", client.id, err);
            }
        }

        // Temporary auto-reload
        if weapon.bullets_left_in_magazine <= 0 && weapon.is_allowed_to_reload() {
            weapon.reload_started_at = Some(time.absolute_time());
        }

        Self::broadcast_shot_event(client_registry, net, &mut weapon, &mut bullet_transform, &velocity)
    }

    fn broadcast_shot_event(client_registry: &ClientRegistry,
                            net: &mut TransportResource,
                            weapon: &mut Weapon,
                            bullet_transform: &mut Transform,
                            velocity: &Vector2<f32>) {
        let payload = serialize(&PacketType::ShotEvent(ShotEvent {
            position: Point2::new(bullet_transform.translation().x, bullet_transform.translation().y),
            velocity: *velocity,
            bullet_time_limit_secs: weapon.bullet_lifespan_sec(),
        })).expect("ShotEvent's serialization failed");

        client_registry.get_clients().iter().map(|handle| handle.addr).for_each(|addr| {
            net.send_with_requirements(addr,
                                       &payload,
                                       DeliveryRequirement::ReliableSequenced(StreamId::ShotEvent.into()),
                                       UrgencyRequirement::OnTick);
        })
    }

    fn check_reload_finish(time: &Time,
                           client_registry: &ClientRegistry,
                           mut net: &mut TransportResource,
                           weapon: &mut Weapon,
                           client: Option<&Client>,
                           reload_start: &Duration) {
        if time.absolute_time_seconds() >= reload_start.as_secs_f64() + weapon.details.reload_time as f64 {
            weapon.bullets_left_in_magazine = weapon.details.magazine_size;
            weapon.reload_started_at = None;

            if let Some(client) = client {
                if let Err(err) = Self::send_ammo_update(&client.id,
                                                         &client_registry,
                                                         weapon.bullets_left_in_magazine,
                                                         &mut net) {
                    log::error!("Failed to send AmmoUpdate to client {:?}. Error: {}", client.id, err);
                }
            }
        }
    }

    fn selected_slot(input: &Input) -> Option<usize> {
        input.get_selection().and_then(|&select| {
            match select {
                InputFlags::SELECT1 => Some(0_usize),
                InputFlags::SELECT2 => Some(1_usize),
                InputFlags::SELECT3 => Some(2_usize),
                _ => None
            }
        })
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use amethyst_test::prelude::*;
    use amethyst::prelude::{World, WorldExt, Builder};
    use crate::components::{Input, InputFlags, weapon, Velocity, Projectile, Lifespan};
    use std::net::SocketAddr;
    use amethyst::Error;
    use amethyst::core::num::Bounded;
    use westiny_common::deserialize;

    #[test]
    fn broadcast_shot_event() -> anyhow::Result<(), Error>{
        amethyst::start_logger(Default::default());
        let mut client_registry = ClientRegistry::new(3);
        client_registry.add(&SocketAddr::new("111.222.111.222".parse().unwrap(), 9999), "player1")?;
        client_registry.add(&SocketAddr::new("222.111.222.111".parse().unwrap(), 9999), "player2")?;
        client_registry.add(&SocketAddr::new("111.111.111.111".parse().unwrap(), 9999), "player3")?;

        AmethystApplication::blank()
            .with_setup(|world: &mut World| {
                world.register::<Input>();
                world.register::<Transform>();
                world.register::<BoundingCircle>();
                world.register::<Weapon>();

                world.register::<Damage>();
                world.register::<Velocity>();
                world.register::<Projectile>();
                world.register::<Lifespan>();
            })
            .with_setup(|world: &mut World| {
                let input = Input {
                    flags: InputFlags::SHOOT,
                    cursor: Point2::new(0.0, 0.0),
                };

                let gun = weapon::WeaponDetails {
                    damage: 5,
                    bullet_distance_limit: 120.0,
                    fire_rate: f32::max_value(),
                    magazine_size: 6,
                    reload_time: 1.0,
                    spread: 2.0,
                    shot: weapon::Shot::Single,
                    bullet_speed: 200.0,
                    pellet_number: 1,
                };

                world.create_entity()
                    .with(input)
                    .with(Transform::default())
                    .with(BoundingCircle { radius: 1.0 })
                    .with(Weapon::new(gun))
                    .build();
            })
            .with_resource(client_registry)
            .with_resource(TransportResource::new())
            .with_system(ShooterSystem, "shooter", &[])
            .with_assertion(|world: &mut World| {
                let net = world.fetch_mut::<TransportResource>();
                let messages = net.get_messages();

                assert_eq!(3, messages.len());
                let expected_msg = ShotEvent{
                    position: Point2::new(0.0, -1.0),
                    velocity: Vector2::new(0.0, -200.0),
                    bullet_time_limit_secs: 0.6
                };

                messages.iter().for_each(|msg| {
                    let deserialized = deserialize(&msg.payload).expect("failed to deserailize");
                    if let PacketType::ShotEvent(ev) = deserialized {
                        // could not apply '==' on PacketType
                        assert_eq!(ev.position, expected_msg.position);
                        assert_eq!(ev.velocity, expected_msg.velocity);
                        assert_eq!(ev.bullet_time_limit_secs, expected_msg.bullet_time_limit_secs);
                    } else {
                        panic!("Unexpected message");
                    }
                })
            })
            .run()
    }
}
