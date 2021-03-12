<<<<<<< HEAD
use amethyst::ecs::{Read, System, ReadStorage, ReadExpect, Entities, WriteStorage, WriteExpect};
=======
use amethyst::derive::SystemDesc;
use amethyst::ecs::{Read, System, SystemData, ReadStorage, ReadExpect, Entities, WriteStorage, WriteExpect};
>>>>>>> 12ef34a (Add handle server's ShotEvent on client)
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

#[cfg(test)]
mod test {
    use super::*;
    use amethyst_test::prelude::*;
    use amethyst::prelude::{World, WorldExt, Builder};
    use crate::components::{Input, InputFlags, weapon, Velocity, Projectile, TimeLimit};
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
                world.register::<Player>();
                world.register::<Input>();
                world.register::<Transform>();
                world.register::<BoundingCircle>();
                world.register::<Weapon>();

                world.register::<Damage>();
                world.register::<Velocity>();
                world.register::<Projectile>();
                world.register::<TimeLimit>();
            })
            .with_setup(|world: &mut World| {
                let input = Input {
                    flags: InputFlags::SHOOT,
                    cursor: Point2::new(0.0, 0.0),
                };

                let gun = weapon::WeaponDetails {
                    damage: 5,
                    bullet_time_limit: 0.6,
                    fire_rate: f32::max_value(),
                    magazine_size: 6,
                    reload_time: 1.0,
                    spread: 2.0,
                    shot: weapon::Shot::Single,
                    bullet_speed: 200.0,
                };

                world.create_entity()
                    .with(Player)
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