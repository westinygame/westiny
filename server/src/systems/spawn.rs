use crate::components;
use crate::resources::ClientRegistry;
use bevy::prelude::*;
use westiny_common::collision;
use westiny_common::events::EntityDelete;
use westiny_common::metric_dimension::{
    length::{Meter, MeterVec2},
    MeterPerSec, Second,
};

pub fn respawn_player(
    mut commands: Commands,
    query: Query<(
        Entity,
        &components::Respawn,
        &components::Eliminated,
        &components::NetworkId,
        &components::Client,
        Option<&Transform>,
    )>,
    time: Res<Time>,
    mut spawn_player_event: EventWriter<SpawnPlayerEvent>,
    mut entity_delete_event: EventWriter<EntityDelete>,
) {
    for (entity, &respawn, &eliminate, &net_id, &client, maybe_transform) in query.iter() {
        if maybe_transform.is_some() {
            // has not been removed yet
            // create a new entity that is waiting until respawn time expires.
            commands
                .spawn()
                .insert(respawn)
                .insert(eliminate)
                .insert(net_id)
                .insert(client);
        } else {
            // we're waiting for respawn time expiration
            if time.seconds_since_startup() - eliminate.elimination_time_sec
                >= respawn.respawn_duration.as_secs_f64()
            {
                // if expired

                log::debug!("Request player spawn");
                spawn_player_event.send(SpawnPlayerEvent {
                    client: client,
                    network_id: net_id,
                });

                entity_delete_event.send(EntityDelete { entity_id: entity });
            }
        }
    }
}

pub fn spawn_player(
    mut commands: Commands,
    mut spawn_player_ec: EventReader<SpawnPlayerEvent>,
    client_registry: Res<ClientRegistry>,
    mut transforms_boundings_query: Query<(&Transform, &components::BoundingCircle)>,
) {
    for spawn_event in spawn_player_ec.iter() {
        let spawn_pos = find_spawn_pos(&mut transforms_boundings_query);
        info!(
            "Spawn position found for player at ({},{})",
            spawn_pos.x.0, spawn_pos.y.0
        );
        create_player_entity(
            &spawn_pos,
            &mut commands,
            spawn_event.client,
            spawn_event.network_id,
            // &gun_resource
        );
        info!(
            "Player created for {}",
            client_registry
                .find_client(spawn_event.client.id)
                .unwrap()
                .player_name
        );
    }
}

fn dummy_guns() -> [(components::weapon::Weapon, &'static str); 3] {
    use components::weapon::{Shot, Weapon, WeaponDetails};
    const REVOLVER: WeaponDetails = WeaponDetails {
        fire_rate: 7.2,
        magazine_size: 6,
        reload_time: Second(2.0),
        damage: 20,
        spread: 10.0,
        bullet_distance_limit: Meter(7.5),
        bullet_speed: MeterPerSec(12.5),
        shot: Shot::Single,
        pellet_number: 1,
    };

    [
        (Weapon::new(REVOLVER), "Revolver"),
        (Weapon::new(REVOLVER), "Another revolver"),
        (Weapon::new(REVOLVER), "One more revolver"),
    ]
}

fn create_player_entity(
    initial_pos: &MeterVec2,
    commands: &mut Commands,
    client: components::Client,
    network_id: components::NetworkId,
    // gun_resource: &GunResource,
) {
    commands
        .spawn()
        .insert(client)
        .insert(network_id)
        .insert(components::Player)
        .insert(Transform::from_xyz(
            initial_pos.x.into_pixel(),
            initial_pos.y.into_pixel(),
            0.0,
        ))
        .insert(GlobalTransform::identity())
        .insert(components::Health(100))
        .insert(components::Input::default())
        .insert(components::Velocity::default())
        .insert(components::BoundingCircle { radius: Meter(0.5) })
        .insert(components::weapon::Holster::new_with_guns(dummy_guns()))
        .insert(components::Respawn {
            respawn_duration: std::time::Duration::from_secs(5),
        });
}

fn has_collision(
    transforms_boundings_query: &mut Query<(&Transform, &components::BoundingCircle)>,
    collider: &collision::Collider,
) -> bool {
    for (transform, bound) in transforms_boundings_query.iter() {
        if let Some(_) = collision::check_body_collision(
            collision::Collider { transform, bound },
            collider.clone(),
        ) {
            return true;
        }
    }
    false
}

fn find_spawn_pos(
    transforms_boundings_query: &mut Query<(&Transform, &components::BoundingCircle)>,
) -> MeterVec2 {
    use rand::Rng;

    // TODO Quick 'n' dirty stuff
    const MAX_TRIAL_ITERATION: u32 = 1024;
    const MAP_SIZE: u32 = 64;
    const TILE_SIZE: u32 = 16;
    const BOUND: f32 = (MAP_SIZE / 2 * TILE_SIZE) as f32;

    let candidate_bounding = components::BoundingCircle { radius: Meter(0.5) };

    for _ in 0..MAX_TRIAL_ITERATION {
        // TODO hardcoded range: should be calculated from map data
        let x = rand::thread_rng().gen_range(-BOUND..BOUND);
        let y = rand::thread_rng().gen_range(-BOUND..BOUND);

        let candidate_transform = Transform::from_xyz(x, y, 0.0);
        if !has_collision(
            transforms_boundings_query,
            &collision::Collider {
                transform: &candidate_transform,
                bound: &candidate_bounding,
            },
        ) {
            return MeterVec2::from_pixel_vec(Vec2::new(x, y));
        }
    }

    log::warn!("Could not find a valid spawn place for player! Fallback to (0,0)");
    MeterVec2::from_raw(0.0, 0.0)
}

pub struct SpawnPlayerEvent {
    pub client: components::Client,
    pub network_id: components::NetworkId,
}

// #[cfg(test)]
// mod test {
//     use super::*;
//     use crate::components::{Client, EntityType, Health, Respawn, BoundingCircle, Input, NetworkId, Player, Velocity,
//     };
//     use amethyst::ecs::prelude::*;
//     use amethyst::ecs::World;
//     use amethyst::core::Transform;
//     use crate::resources::ClientID;
//     use amethyst::utils::application_root_dir;
//     use crate::components::weapon::Holster;
//
//     fn create_testworld() -> World {
//         let mut world = World::new();
//         world.register::<Transform>();
//         world.register::<Client>();
//         world.register::<NetworkId>();
//         world.register::<Player>();
//         world.register::<Input>();
//         world.register::<Velocity>();
//         world.register::<BoundingCircle>();
//         world.register::<Health>();
//         world.register::<Respawn>();
//         world.register::<Holster>();
//
//         let resources_path = application_root_dir().unwrap().join("../resources");
//
//         GunResource::initialize(&mut world, &resources_path).expect(&format!("Resources path: {}", resources_path.as_os_str().to_str().unwrap()));
//         world
//     }
//
//     #[test]
//     fn spawn_one_player() {
//         let cli_id = ClientID(42);
//
//         let mut world = create_testworld();
//         {
//             SpawnSystem::spawn_player(
//                 &Point2::new(0.0, 0.0),
//                 &world.entities(),
//                 Client{id: cli_id},
//                 NetworkId {id: 0, entity_type: EntityType::Player},
//                 &world.read_resource::<GunResource>(),
//                 &world.read_resource::<LazyUpdate>(),
//             );
//         }
//         world.maintain();
//
//         let entities: Vec<_> = world.entities().join().collect();
//         assert_eq!(entities.len(), 1);
//         assert_eq!(
//             world.read_storage::<Client>().get(entities[0]).unwrap().id,
//             cli_id
//         );
//     }
//
//     #[test]
//     fn spawn_two_player() {
//         let mut world = create_testworld();
//         {
//             SpawnSystem::spawn_player(
//                 &Point2::new(0.0, 0.0),
//                 &world.entities(),
//                 Client {id: ClientID(42)},
//                 NetworkId {id: 0, entity_type: EntityType::Player},
//                 &world.read_resource::<GunResource>(),
//                 &world.read_resource::<LazyUpdate>(),
//             );
//             SpawnSystem::spawn_player(
//                 &Point2::new(0.0, 0.0),
//                 &world.entities(),
//                 Client {id: ClientID(43)},
//                 NetworkId { id: 1, entity_type: EntityType::Player},
//                 &world.read_resource::<GunResource>(),
//                 &world.read_resource::<LazyUpdate>(),
//             );
//         }
//         world.maintain();
//
//         use std::collections::BTreeSet;
//
//         let cli_storage = world.read_storage::<Client>();
//         let client_ids: BTreeSet<_> = world
//             .entities()
//             .join()
//             .map(|e| cli_storage.get(e).unwrap().id.0)
//             .collect();
//
//         assert!(client_ids.contains(&42));
//         assert!(client_ids.contains(&43));
//     }
//
//     // TODO this logic has been moved to ClientIntroductionSystem. Domi, pls move this testcase there
//     // #[test]
//     // fn respawn_player_should_not_create_new_entity() {
//     //     let mut world = create_testworld();
//     //     let mut net_id_sup = NetworkIdSupplier::new();
//     //     let cli_id = ClientID(42);
//     //
//     //     let first_net_id;
//     //     {
//     //         first_net_id = ClientIntroductionSystem::spawn_player(
//     //             &Point2::new(0.0, 0.0),
//     //             &world.entities(),
//     //             &cli_id,
//     //             &world.read_storage::<Client>(),
//     //             &world.read_storage::<NetworkId>(),
//     //             &mut net_id_sup,
//     //             &world.read_resource::<LazyUpdate>(),
//     //         );
//     //     }
//     //     world.maintain();
//     //     {
//     //         let second_net_id = ClientIntroductionSystem::spawn_player(
//     //             &Point2::new(0.0, 0.0),
//     //             &world.entities(),
//     //             &cli_id,
//     //             &world.read_storage::<Client>(),
//     //             &world.read_storage::<NetworkId>(),
//     //             &mut net_id_sup,
//     //             &world.read_resource::<LazyUpdate>(),
//     //         );
//     //         assert_eq!(first_net_id, second_net_id);
//     //     }
//     //     world.maintain();
//     //
//     //     let entities: Vec<_> = world.entities().join().collect();
//     //     assert_eq!(entities.len(), 1);
//     //     assert_eq!(
//     //         world.read_storage::<Client>().get(entities[0]).unwrap().id,
//     //         cli_id
//     //     );
//     // }
// }
