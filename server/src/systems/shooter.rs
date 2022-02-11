use crate::components::{
    weapon::Holster, weapon::Weapon, BoundingCircle, Client, Input, InputFlags,
};
use crate::resources::{ClientID, ClientRegistry, StreamId};
use bevy::prelude::{Commands, Query, Res, ResMut, SystemSet, Time, Transform, Vec3};
use blaminar::simulation::{DeliveryRequirement, TransportResource, UrgencyRequirement};
use std::f32::consts::PI;
use westiny_common::entities::spawn_bullet;
use westiny_common::metric_dimension::{length::MeterVec2, MeterPerSecVec2};
use westiny_common::network::{PacketType, PlayerUpdate, ShotEvent};
use westiny_common::serialization::serialize;

pub fn weapon_handler_system_set() -> SystemSet {
    SystemSet::new()
        .label("weapon_handler")
        .with_system(weapon_switcher::switch_weapon)
        .with_system(reloader::reload)
        .with_system(shooter::shoot)
}

mod weapon_switcher {
    use super::*;

    pub fn switch_weapon(
        time: Res<Time>,
        client_registry: Res<ClientRegistry>,
        mut net: ResMut<TransportResource>,
        mut input_query: Query<(&Input, &mut Holster, Option<&Client>)>,
    ) {
        for (input, mut holster, maybe_client) in input_query.iter_mut() {
            if let Some(selected_slot) = selected_slot(&input) {
                if holster.active_slot() == selected_slot {
                    continue;
                }

                if let Some(gun_name) = holster.switch(selected_slot) {
                    let gun = holster.active_gun_mut();
                    if gun.reload_started_at.is_some() {
                        // if last switch from this happened mid-reload, restart it
                        gun.reload_started_at = Some(time.time_since_startup());
                    }

                    if let Some(client) =
                        maybe_client.and_then(|client| client_registry.find_client(client.id))
                    {
                        let payload_packet = PacketType::PlayerUpdate(PlayerUpdate::WeaponSwitch {
                            name: gun_name.to_string(),
                            magazine_size: gun.details.magazine_size,
                            ammo_in_magazine: gun.bullets_left_in_magazine,
                        });

                        let payload = serialize(&payload_packet).expect(&format!(
                            "Failed to serialize 'WeaponSwitch' packet: {:?}",
                            payload_packet
                        ));
                        net.send_with_requirements(
                            client.addr,
                            &payload,
                            DeliveryRequirement::ReliableSequenced(StreamId::WeaponSwitch.into()),
                            UrgencyRequirement::OnTick,
                        );
                    }
                }
            }
        }
    }

    fn selected_slot(input: &Input) -> Option<usize> {
        input.get_selection().and_then(|&select| match select {
            InputFlags::SELECT1 => Some(0_usize),
            InputFlags::SELECT2 => Some(1_usize),
            InputFlags::SELECT3 => Some(2_usize),
            _ => None,
        })
    }
}

fn send_ammo_update(
    client_id: &ClientID,
    client_registry: &ClientRegistry,
    ammo_in_magazine: u32,
    net: &mut TransportResource,
) -> anyhow::Result<()> {
    let payload = serialize(&PacketType::PlayerUpdate(PlayerUpdate::AmmoUpdate {
        ammo_in_magazine,
    }))
    .map_err(|err| anyhow::anyhow!("Failed to serialize AmmoUpdate: {}", err))?;
    let address = client_registry
        .find_client(*client_id)
        .map(|handle| handle.addr)
        .ok_or(anyhow::anyhow!(
            "Client with id {:?} not found in registry",
            client_id
        ))?;
    net.send_with_requirements(
        address,
        &payload,
        DeliveryRequirement::ReliableSequenced(StreamId::AmmoUpdate.into()),
        UrgencyRequirement::OnTick,
    );
    Ok(())
}

mod reloader {
    use super::*;

    pub fn reload(
        time: Res<Time>,
        client_registry: Res<ClientRegistry>,
        mut net: ResMut<TransportResource>,
        mut input_query: Query<(&Input, &mut Holster, Option<&Client>)>,
    ) {
        for (&input, mut holster, maybe_client) in input_query.iter_mut() {
            let mut weapon = holster.active_gun_mut();
            if input.flags.intersects(InputFlags::RELOAD) && weapon.is_allowed_to_reload() {
                weapon.reload_started_at = Some(time.time_since_startup())
            } else if let Some(reload_start) = weapon.reload_started_at {
                check_reload_finish(
                    &time,
                    &client_registry,
                    &mut net,
                    weapon,
                    maybe_client,
                    &reload_start,
                )
            }
        }
    }

    fn check_reload_finish(
        time: &Time,
        client_registry: &ClientRegistry,
        mut net: &mut TransportResource,
        weapon: &mut Weapon,
        client: Option<&Client>,
        reload_start: &std::time::Duration,
    ) {
        if time.seconds_since_startup()
            >= reload_start.as_secs_f64() + weapon.details.reload_time.0 as f64
        {
            weapon.bullets_left_in_magazine = weapon.details.magazine_size;
            weapon.reload_started_at = None;

            if let Some(client) = client {
                if let Err(err) = send_ammo_update(
                    &client.id,
                    &client_registry,
                    weapon.bullets_left_in_magazine,
                    &mut net,
                ) {
                    log::error!(
                        "Failed to send AmmoUpdate to client {:?}. Error: {}",
                        client.id,
                        err
                    );
                }
            }
        }
    }
}

mod shooter {
    use super::*;

    pub fn shoot(
        mut commands: Commands,
        time: Res<Time>,
        client_registry: Res<ClientRegistry>,
        mut net: ResMut<TransportResource>,
        mut query: Query<(
            &Input,
            &Transform,
            Option<&BoundingCircle>,
            &mut Holster,
            Option<&Client>,
        )>,
    ) {
        for (input, shooter_transform, maybe_bound, mut holster, maybe_client) in query.iter_mut() {
            let mut weapon = holster.active_gun_mut();
            if input.flags.intersects(InputFlags::SHOOT) {
                if weapon.is_allowed_to_shoot(time.time_since_startup()) {
                    let mut bullet_transform = Transform::default();
                    bullet_transform.translation = shooter_transform.translation;
                    bullet_transform.rotation = shooter_transform.rotation;

                    let mut direction3d = Vec3::Y;
                    westiny_common::utilities::rotate_vec3_around_z(
                        &bullet_transform.rotation,
                        &mut direction3d,
                    );
                    if let Some(bound) = maybe_bound {
                        bullet_transform.translation =
                            bullet_transform.translation - bound.radius.into_pixel() * direction3d;
                    }

                    for _pellet_idx in 0..weapon.details.pellet_number {
                        let velocity_direction = spread_to_quat(weapon.details.spread)
                            .mul_vec3(direction3d)
                            .truncate()
                            * -1.0;
                        let velocity = weapon.details.bullet_speed * velocity_direction;

                        spawn_bullet(
                            &mut commands,
                            weapon.details.damage,
                            bullet_transform.clone(),
                            velocity,
                            time.time_since_startup(),
                            weapon.bullet_lifespan_sec(),
                        );

                        broadcast_shot_event(
                            &client_registry,
                            &mut net,
                            &weapon,
                            &bullet_transform,
                            &velocity,
                        );
                    }

                    weapon.last_shot_time = time.time_since_startup();
                    weapon.input_lifted = false;
                    weapon.bullets_left_in_magazine -= 1;

                    if let Some(client) = maybe_client {
                        if let Err(err) = send_ammo_update(
                            &client.id,
                            &client_registry,
                            weapon.bullets_left_in_magazine,
                            &mut net,
                        ) {
                            bevy::log::error!(
                                "Failed to send ammo update to client {:?}. Error: {}",
                                client.id,
                                err
                            );
                        }
                    }
                }
            } else {
                weapon.input_lifted = true;
            }
        }
    }

    fn spread_to_quat(spread: f32) -> bevy::math::Quat {
        let angle = if spread > 0.0 {
            use rand::Rng;
            rand::thread_rng().gen_range(-spread..spread) * (PI / 180.0)
        } else {
            0.0
        };

        bevy::math::Quat::from_rotation_z(angle)
    }

    fn broadcast_shot_event(
        client_registry: &ClientRegistry,
        net: &mut TransportResource,
        weapon: &Weapon,
        bullet_transform: &Transform,
        velocity: &MeterPerSecVec2,
    ) {
        let payload = serialize(&PacketType::ShotEvent(ShotEvent {
            position: MeterVec2::from_pixel_vec(bullet_transform.translation.truncate()),
            velocity: *velocity,
            bullet_time_limit_secs: weapon.bullet_lifespan_sec(),
        }))
        .expect("ShotEvent's serialization failed");

        client_registry
            .get_clients()
            .iter()
            .map(|handle| handle.addr)
            .for_each(|addr| {
                net.send_with_requirements(
                    addr,
                    &payload,
                    DeliveryRequirement::ReliableSequenced(StreamId::ShotEvent.into()),
                    UrgencyRequirement::OnTick,
                );
            })
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::components::weapon::WeaponDetails;
    use crate::components::{weapon, Input, InputFlags};
    use bevy::prelude::{App, Commands, Transform};
    use std::net::SocketAddr;
    use w_bevy_test::TestApp;
    use westiny_common::metric_dimension::length::Meter;
    use westiny_common::metric_dimension::MeterPerSec;
    use westiny_common::metric_dimension::Second;

    fn spawn_shooting_player(mut commands: Commands) {
        let input = Input {
            flags: InputFlags::SHOOT,
            cursor: MeterVec2::from_raw(0.0, 0.0),
        };

        let gun = WeaponDetails {
            damage: 5,
            bullet_distance_limit: Meter(7.5),
            fire_rate: f32::MAX,
            magazine_size: 6,
            reload_time: Second(1.0),
            spread: 0.0,
            shot: weapon::Shot::Single,
            bullet_speed: MeterPerSec(12.5),
            pellet_number: 1,
        };

        let guns = [
            (Weapon::new(gun.clone()), "Weapon1"),
            (Weapon::new(gun.clone()), "Weapon2"),
            (Weapon::new(gun), "Weapon3"),
        ];

        commands
            .spawn()
            .insert(input)
            .insert(Transform::default())
            .insert(BoundingCircle { radius: Meter(1.0) })
            .insert(Holster::new_with_guns(guns));
    }

    #[test]
    fn broadcast_shot_event() {
        use westiny_common::serialization::deserialize;

        let mut client_registry = ClientRegistry::new(3);
        client_registry
            .add(
                &SocketAddr::new("111.222.111.222".parse().unwrap(), 9999),
                "player1",
            )
            .unwrap();
        client_registry
            .add(
                &SocketAddr::new("222.111.222.111".parse().unwrap(), 9999),
                "player2",
            )
            .unwrap();
        client_registry
            .add(
                &SocketAddr::new("111.111.111.111".parse().unwrap(), 9999),
                "player3",
            )
            .unwrap();

        let mut time = bevy::prelude::Time::default();
        time.update();

        App::new()
            .insert_resource(client_registry)
            .insert_resource(TransportResource::new())
            .insert_resource(time)
            .add_startup_system(spawn_shooting_player)
            .add_system(shooter::shoot)
            .add_assert_system(|net: ResMut<TransportResource>| {
                let messages = net.get_messages();

                assert_eq!(3, messages.len());
                let expected_msg = ShotEvent {
                    position: MeterVec2::from_raw(0.0, -1.0),
                    velocity: MeterPerSecVec2::from_raw(0.0, -12.5),
                    bullet_time_limit_secs: Second(0.6),
                };

                messages.iter().for_each(|msg| {
                    let deserialized = deserialize(&msg.payload).expect("failed to deserailize");
                    if let PacketType::ShotEvent(ev) = deserialized {
                        assert_eq!(ev.position, expected_msg.position);
                        assert_eq!(ev.velocity, expected_msg.velocity);
                        assert_eq!(
                            ev.bullet_time_limit_secs,
                            expected_msg.bullet_time_limit_secs
                        );
                    } else {
                        panic!("Unexpected message");
                    }
                })
            })
            .run();
    }
}
