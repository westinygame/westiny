use amethyst::derive::SystemDesc;
use amethyst::core::ecs::{System, SystemData, ReadStorage, Read, Join, LazyUpdate, ReadExpect, Entities, Builder,  ReaderId, WriteExpect};
use amethyst::core::{Time, Transform};
use crate::components;
use amethyst::core::math::Point2;
use std::time::Duration;
use westiny_common::collision;
use amethyst::core::ecs::shrev::EventChannel;
use westiny_common::resources::EntityDelete;
use derive_new::new;

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
        ReadStorage<'s, components::Client>,
        ReadStorage<'s, components::NetworkId>,
        ReadExpect<'s, LazyUpdate>,
    );

    fn run(&mut self, data: Self::SystemData) {
        let (
            spawn_event_channel,
            transforms,
            boundings,
            entities,
            clients,
            net_ids,
            lazy,
        ) = data;

        for spawn_event in spawn_event_channel.read(&mut self.reader) {
            let spawn_pos = SpawnSystem::find_spawn_pos(&transforms, &boundings);
            SpawnSystem::spawn_player(&spawn_pos,
                                      &entities,
                                      spawn_event.client,
                                      &clients,
                                      &net_ids,
                                      spawn_event.network_id,
                                      &lazy);
        }
    }
}

impl SpawnSystem {
    fn spawn_player(
        initial_pos: &Point2<f32>,
        entities: &Entities<'_>,
        client: components::Client,
        clients: &ReadStorage<'_, components::Client>,
        network_ids: &ReadStorage<'_, components::NetworkId>,
        network_id: components::NetworkId,
        lazy_update: &LazyUpdate,
    ) -> components::NetworkId {
        use components::weapon;

        if let Some((cli, net_id)) = (clients, network_ids)
            .join()
            .find(|(&cli, _)| cli.id == client.id)
        {
            log::info!(
                "{:?} already connected, its entity already spawned: {:?}",
                cli.id,
                net_id
            );
            return *net_id;
        }

        let transform = {
            let mut t = Transform::default();
            t.set_translation_xyz(initial_pos.x, initial_pos.y, 0.0);
            t
        };

        // TODO define these values in RON resource files. PREFAB?
        let revolver = weapon::WeaponDetails {
            damage: 5,
            distance: 120.0,
            fire_rate: 7.2,
            magazine_size: 6,
            reload_time: 1.0,
            spread: 2.0,
            shot: weapon::Shot::Single,
            bullet_speed: 200.0,
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
            .with(components::weapon::Weapon::new(revolver))
            .with(components::BoundingCircle { radius: 8.0 })
            .with(components::Respawn {respawn_duration: Duration::from_secs(5)})
            .build();

        network_id
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
        // TODO Quick 'n' dirty stuff
        const MAX_TRIAL_ITERATION: u32 = 1024;
        const MAP_SIZE: u32 = 64;
        const TILE_SIZE: u32 = 16;
        const BOUND: f32 = (MAP_SIZE/2 * TILE_SIZE) as f32;
        use rand::Rng;

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
