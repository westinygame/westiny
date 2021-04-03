use amethyst::derive::SystemDesc;
use amethyst::core::ecs::{System, SystemData, ReadStorage, Read, Join, LazyUpdate, ReadExpect, Entities, Builder,  ReaderId, WriteExpect};
use amethyst::core::{Time, Transform};
use crate::components;
use amethyst::core::math::Point2;
use std::time::Duration;
use westiny_common::collision;
use amethyst::core::ecs::shrev::EventChannel;
use westiny_common::events::EntityDelete;
use derive_new::new;
use crate::resources::ClientRegistry;
use westiny_common::resources::weapon::{GunResource, GunId};

pub struct RespawnSystem;

impl<'s> System<'s> for RespawnSystem {
    type SystemData = (
        ReadStorage<'s, components::Respawn>,
        ReadStorage<'s, components::Eliminated>,
        ReadStorage<'s, components::NetworkId>,
        ReadStorage<'s, components::Client>,
        ReadStorage<'s, Transform>,
        Read<'s, Time>,
        ReadExpect<'s, LazyUpdate>,
        Entities<'s>,
        WriteExpect<'s, EventChannel<EntityDelete>>,
        WriteExpect<'s, EventChannel<SpawnPlayerEvent>>,
    );

    fn run(&mut self, data: Self::SystemData) {
        let (
            respawns,
            eliminates,
            net_ids,
            clients,
            transforms,
            time,
            lazy,
            entities,
            mut entity_delete_event_channel,
            mut spawn_player_event_channel,
        ) = data;

        for (respawn, eliminate, net_id, client, opt_transform, entity)
                in (&respawns, &eliminates, &net_ids, &clients, (&transforms).maybe(), &entities).join() {
            if opt_transform.is_some() {
                // has not been removed yet
                // create a new entity that is waiting until respawn time expires.
                lazy.create_entity(&entities)
                    .with(*respawn)
                    .with(*eliminate)
                    .with(*net_id)
                    .with(*client)
                    .build();
            } else {
                // we're waiting for respawn time expiration
                if time.absolute_time_seconds() - eliminate.elimination_time_sec >= respawn.respawn_duration.as_secs_f64() {
                    // if expired

                    log::debug!("Request player spawn");
                    spawn_player_event_channel.single_write(SpawnPlayerEvent {
                        client: *client,
                        network_id: *net_id,
                    });

                    entity_delete_event_channel.single_write(EntityDelete { entity_id: entity });
                }
            }
        }
    }
}

#[derive(SystemDesc, new)]
#[system_desc(name(SpawnSystemDesc))]
pub struct SpawnSystem {
    #[system_desc(event_channel_reader)]
    reader: ReaderId<SpawnPlayerEvent>,
}

impl<'s> System<'s> for SpawnSystem {
    type SystemData = (
        Read<'s, EventChannel<SpawnPlayerEvent>>,
        ReadStorage<'s, Transform>,
        ReadStorage<'s, components::BoundingCircle>,
        Entities<'s>,
        ReadExpect<'s, LazyUpdate>,
        ReadExpect<'s, ClientRegistry>,
        ReadExpect<'s, GunResource>,
    );

    fn run(&mut self, data: Self::SystemData) {
        let (
            spawn_event_channel,
            transforms,
            boundings,
            entities,
            lazy,
            client_registry,
            gun_resource,
        ) = data;

        for spawn_event in spawn_event_channel.read(&mut self.reader) {
            let spawn_pos = SpawnSystem::find_spawn_pos(&transforms, &boundings);
            SpawnSystem::spawn_player(&spawn_pos,
                                      &entities,
                                      spawn_event.client,
                                      spawn_event.network_id,
                                      &gun_resource,
                                      &lazy);
            log::info!("Player created for {}", client_registry.find_client(spawn_event.client.id).unwrap().player_name);
        }
    }
}

impl SpawnSystem {
    fn spawn_player(
        initial_pos: &Point2<f32>,
        entities: &Entities<'_>,
        client: components::Client,
        network_id: components::NetworkId,
        gun_resource: &GunResource,
        lazy_update: &LazyUpdate,
    ) {
        let transform = {
            let mut t = Transform::default();
            t.set_translation_xyz(initial_pos.x, initial_pos.y, 0.0);
            t
        };

        lazy_update
            .create_entity(entities)
            .with(client)
            .with(network_id)
            .with(components::Player)
            .with(transform)
            .with(components::Health(100))
            .with(components::Input::default())
            .with(components::Velocity::default())
            .with(components::weapon::Weapon::new(gun_resource.get_gun(GunId::Revolver)))
            .with(components::BoundingCircle { radius: 8.0 })
            .with(components::Respawn {respawn_duration: Duration::from_secs(5)})
            .build();
    }

    fn has_collision(
        transform_storage: &ReadStorage<'_, Transform>,
        bounding_storage: &ReadStorage<'_, components::BoundingCircle>,
        collider: &collision::Collider
    ) -> bool {
        for (transform, bound) in (transform_storage, bounding_storage).join() {
            if let Some(_) = collision::check_body_collision(
                collision::Collider{transform, bound},
                collider.clone())
            {
                return true;
            }
        }
        false
    }

    fn find_spawn_pos(
        transform_storage: &ReadStorage<'_, Transform>,
        bounding_storage: &ReadStorage<'_, components::BoundingCircle>
    ) -> Point2<f32> {
        use rand::Rng;

        // TODO Quick 'n' dirty stuff
        const MAX_TRIAL_ITERATION: u32 = 1024;
        const MAP_SIZE: u32 = 64;
        const TILE_SIZE: u32 = 16;
        const BOUND: f32 = (MAP_SIZE/2 * TILE_SIZE) as f32;

        let candidate_bounding = components::BoundingCircle { radius: 8.0 };

        for _ in 0..MAX_TRIAL_ITERATION {
            // TODO hardcoded range: should be calculated from map data
            let x = rand::thread_rng().gen_range(-BOUND .. BOUND);
            let y = rand::thread_rng().gen_range(-BOUND .. BOUND);

            let candidate_transform = {
                let mut t = Transform::default();
                t.set_translation_xyz(x, y, 0.0);
                t
            };
            if !Self::has_collision(
                transform_storage, bounding_storage,
                &collision::Collider{transform: &candidate_transform, bound: &candidate_bounding}
            ) {
                return Point2::new(x, y);
            }
        }

        log::warn!("Could not find a valid spawn place for player! Fallback to (0,0)");
        Point2::new(0.0, 0.0)
    }
}

pub struct SpawnPlayerEvent {
    pub client: components::Client,
    pub network_id: components::NetworkId,
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::components::{Client, EntityType, Health, Respawn, weapon::Weapon, BoundingCircle, Input, NetworkId, Player, Velocity,
    };
    use amethyst::ecs::prelude::*;
    use amethyst::ecs::World;
    use amethyst::core::Transform;
    use crate::resources::ClientID;
    use amethyst::utils::application_root_dir;

    fn create_testworld() -> World {
        let mut world = World::new();
        world.register::<Transform>();
        world.register::<Client>();
        world.register::<NetworkId>();
        world.register::<Player>();
        world.register::<Input>();
        world.register::<Velocity>();
        world.register::<Weapon>();
        world.register::<BoundingCircle>();
        world.register::<Health>();
        world.register::<Respawn>();

        let resources_path = application_root_dir().unwrap().join("../resources");

        GunResource::initialize(&mut world, &resources_path).expect(&format!("Resources path: {}", resources_path.as_os_str().to_str().unwrap()));
        world
    }

    #[test]
    fn spawn_one_player() {
        let cli_id = ClientID(42);

        let mut world = create_testworld();
        {
            SpawnSystem::spawn_player(
                &Point2::new(0.0, 0.0),
                &world.entities(),
                Client{id: cli_id},
                NetworkId {id: 0, entity_type: EntityType::Player},
                &world.read_resource::<GunResource>(),
                &world.read_resource::<LazyUpdate>(),
            );
        }
        world.maintain();

        let entities: Vec<_> = world.entities().join().collect();
        assert_eq!(entities.len(), 1);
        assert_eq!(
            world.read_storage::<Client>().get(entities[0]).unwrap().id,
            cli_id
        );
    }

    #[test]
    fn spawn_two_player() {
        let mut world = create_testworld();
        {
            SpawnSystem::spawn_player(
                &Point2::new(0.0, 0.0),
                &world.entities(),
                Client {id: ClientID(42)},
                NetworkId {id: 0, entity_type: EntityType::Player},
                &world.read_resource::<GunResource>(),
                &world.read_resource::<LazyUpdate>(),
            );
            SpawnSystem::spawn_player(
                &Point2::new(0.0, 0.0),
                &world.entities(),
                Client {id: ClientID(43)},
                NetworkId { id: 1, entity_type: EntityType::Player},
                &world.read_resource::<GunResource>(),
                &world.read_resource::<LazyUpdate>(),
            );
        }
        world.maintain();

        use std::collections::BTreeSet;

        let cli_storage = world.read_storage::<Client>();
        let client_ids: BTreeSet<_> = world
            .entities()
            .join()
            .map(|e| cli_storage.get(e).unwrap().id.0)
            .collect();

        assert!(client_ids.contains(&42));
        assert!(client_ids.contains(&43));
    }

    // TODO this logic has been moved to ClientIntroductionSystem. Domi, pls move this testcase there
    // #[test]
    // fn respawn_player_should_not_create_new_entity() {
    //     let mut world = create_testworld();
    //     let mut net_id_sup = NetworkIdSupplier::new();
    //     let cli_id = ClientID(42);
    //
    //     let first_net_id;
    //     {
    //         first_net_id = ClientIntroductionSystem::spawn_player(
    //             &Point2::new(0.0, 0.0),
    //             &world.entities(),
    //             &cli_id,
    //             &world.read_storage::<Client>(),
    //             &world.read_storage::<NetworkId>(),
    //             &mut net_id_sup,
    //             &world.read_resource::<LazyUpdate>(),
    //         );
    //     }
    //     world.maintain();
    //     {
    //         let second_net_id = ClientIntroductionSystem::spawn_player(
    //             &Point2::new(0.0, 0.0),
    //             &world.entities(),
    //             &cli_id,
    //             &world.read_storage::<Client>(),
    //             &world.read_storage::<NetworkId>(),
    //             &mut net_id_sup,
    //             &world.read_resource::<LazyUpdate>(),
    //         );
    //         assert_eq!(first_net_id, second_net_id);
    //     }
    //     world.maintain();
    //
    //     let entities: Vec<_> = world.entities().join().collect();
    //     assert_eq!(entities.len(), 1);
    //     assert_eq!(
    //         world.read_storage::<Client>().get(entities[0]).unwrap().id,
    //         cli_id
    //     );
    // }
}
