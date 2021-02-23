use amethyst::{
    core::{math::Point2, Transform},
    derive::SystemDesc,
    ecs::{
        Entities, Join, LazyUpdate, Read, ReadExpect, ReadStorage, System, SystemData, WriteExpect,
    },
    network::simulation::{DeliveryRequirement, TransportResource, UrgencyRequirement},
    prelude::Builder,
    shrev::{EventChannel, ReaderId},
};

use anyhow::Result;
use derive_new::new;

use westiny_common::{
    network::{ClientInitialData, PacketType},
    serialize,
};

use crate::{
    components,
    components::EntityType,
    resources::{ClientID, ClientNetworkEvent, ClientRegistry, NetworkIdSupplier},
};
use westiny_common::resources::Seed;

#[derive(SystemDesc, new)]
#[system_desc(name(ClientIntroductionSystemDesc))]
pub struct ClientIntroductionSystem {
    #[system_desc(event_channel_reader)]
    reader: ReaderId<ClientNetworkEvent>,
}

impl<'s> System<'s> for ClientIntroductionSystem {
    type SystemData = (
        Read<'s, EventChannel<ClientNetworkEvent>>,
        Entities<'s>,
        WriteExpect<'s, TransportResource>,
        ReadExpect<'s, ClientRegistry>,
        ReadExpect<'s, Seed>,
        WriteExpect<'s, NetworkIdSupplier>,
        ReadExpect<'s, LazyUpdate>,
        ReadStorage<'s, components::Client>,
        ReadStorage<'s, components::NetworkId>,
    );

    fn run(
        &mut self,
        (
            client_net_ec,
            entities,
            mut net,
            client_registry,
            seed,
            mut net_id_supplier,
            lazy_update,
            client,
            network_ids,
        ): Self::SystemData,
    ) {
        // This vector is used for deduplicating ClientConnected events within one frame to avoid
        // multiple spawn for a single client.
        let mut added_clients = Vec::<(ClientID, components::NetworkId)>::new();

        for client_network_event in client_net_ec.read(&mut self.reader) {
            match client_network_event {
                ClientNetworkEvent::ClientConnected(client_id) => {
                    let client_handle = client_registry.find_client(*client_id).expect(&format!(
                        "Client [client_id: {:?}] not found in registry",
                        client_id
                    ));

                    let entity_network_id = if let Some((_, net_id)) =
                        added_clients.iter().find(|(cli_id, _)| cli_id == client_id)
                    {
                        log::info!(
                            "Player for {:?} already spawned: {:?}, not respawning.",
                            client_id,
                            net_id
                        );
                        *net_id
                    } else {
                        let net_id = Self::spawn_player(
                            &entities,
                            client_id,
                            &client,
                            &network_ids,
                            &mut net_id_supplier,
                            &lazy_update,
                        );
                        log::debug!(
                            "Player entity spawned for {}, {:?}, {:?}",
                            client_handle.player_name,
                            client_id,
                            net_id
                        );
                        added_clients.push((*client_id, net_id));
                        net_id
                    };

                    // Send response to client
                    let connection_response =
                        PacketType::ConnectionResponse(Ok(ClientInitialData {
                            player_network_id: entity_network_id,
                            initial_pos: Point2::from([0.0, 0.0]),
                            seed: *seed
                        })
                    );
                    net.send_with_requirements(
                        client_handle.addr,
                        &serialize(&connection_response).unwrap(),
                        DeliveryRequirement::Reliable,
                        UrgencyRequirement::OnTick,
                    )
                }
                ClientNetworkEvent::ClientDisconnected(client_id) => {
                    match Self::despawn_player(&entities, &client, client_id) {
                        Ok(()) => log::debug!("Disconnecting client's player entity [client_id: {:?}], has been removed", client_id),
                        Err(err) => log::error!("{}", err)
                    }
                }
            }
        }
    }
}

impl ClientIntroductionSystem {
    fn spawn_player(
        entities: &Entities<'_>,
        client_id: &ClientID,
        clients: &ReadStorage<'_, components::Client>,
        network_ids: &ReadStorage<'_, components::NetworkId>,
        net_id_supplier: &mut NetworkIdSupplier,
        lazy_update: &LazyUpdate,
    ) -> components::NetworkId {
        use components::weapon;

        if let Some((cli, net_id)) = (clients, network_ids)
            .join()
            .find(|(&cli, _)| &cli.id == client_id)
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
            t.set_translation_xyz(0.0, 0.0, 0.0);
            t
        };

        // TODO define these values in RON resource files. PREFAB?
        let revolver = weapon::WeaponDetails {
            damage: 5.0,
            distance: 120.0,
            fire_rate: 7.2,
            magazine_size: 6,
            reload_time: 1.0,
            spread: 2.0,
            shot: weapon::Shot::Single,
            bullet_speed: 200.0,
        };

        let client = components::Client::new(*client_id);
        let network_id = net_id_supplier.next(EntityType::Player);

        lazy_update
            .create_entity(entities)
            .with(client)
            .with(network_id)
            .with(components::Player)
            .with(transform)
            .with(components::Input::default())
            .with(components::Velocity::default())
            .with(components::weapon::Weapon::new(revolver))
            .with(components::BoundingCircle { radius: 8.0 })
            .build();

        network_id
    }

    fn despawn_player(
        entities: &Entities<'_>,
        client_storage: &ReadStorage<'_, components::Client>,
        client_id: &ClientID,
    ) -> Result<()> {
        for (entity, client) in (&*entities, client_storage).join() {
            if client.id() == client_id {
                return match entities.delete(entity) {
                    Ok(_) =>  Ok(()),
                    Err(err) => Err(anyhow::anyhow!(
                        "Disconnecting client's player entity [client_id: {:?}] could not be removed. {}",
                        client_id,
                        err
                    ))
                };
            }
        }

        Err(anyhow::anyhow!(
        "Disconnecting client's player entity [client_id: {:?}] not found thus could not be removed",
        client_id))
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::components::Client;
    use amethyst::ecs::prelude::*;
    use amethyst::ecs::World;
    use westiny_common::components::{
        weapon::Weapon, BoundingCircle, Input, NetworkId, Player, Velocity,
    };

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
        world
    }

    #[test]
    fn spawn_one_player() {
        let cli_id = ClientID(42);

        let mut world = create_testworld();
        let mut net_id_sup = NetworkIdSupplier::new();
        {
            let clients = world.read_storage::<Client>();
            let net_ids = world.read_storage::<NetworkId>();
            ClientIntroductionSystem::spawn_player(
                &world.entities(),
                &cli_id,
                &clients,
                &net_ids,
                &mut net_id_sup,
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
        let mut net_id_sup = NetworkIdSupplier::new();
        {
            let clients = world.read_storage::<Client>();
            let net_ids = world.read_storage::<NetworkId>();
            let first_net_id = ClientIntroductionSystem::spawn_player(
                &world.entities(),
                &ClientID(42),
                &clients,
                &net_ids,
                &mut net_id_sup,
                &world.read_resource::<LazyUpdate>(),
            );
            let second_net_id = ClientIntroductionSystem::spawn_player(
                &world.entities(),
                &ClientID(43),
                &clients,
                &net_ids,
                &mut net_id_sup,
                &world.read_resource::<LazyUpdate>(),
            );
            assert_ne!(first_net_id, second_net_id);
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

    #[test]
    fn respawn_player_should_not_create_new_entity() {
        let mut world = create_testworld();
        let mut net_id_sup = NetworkIdSupplier::new();
        let cli_id = ClientID(42);

        let first_net_id;
        {
            first_net_id = ClientIntroductionSystem::spawn_player(
                &world.entities(),
                &cli_id,
                &world.read_storage::<Client>(),
                &world.read_storage::<NetworkId>(),
                &mut net_id_sup,
                &world.read_resource::<LazyUpdate>(),
            );
        }
        world.maintain();
        {
            let second_net_id = ClientIntroductionSystem::spawn_player(
                &world.entities(),
                &cli_id,
                &world.read_storage::<Client>(),
                &world.read_storage::<NetworkId>(),
                &mut net_id_sup,
                &world.read_resource::<LazyUpdate>(),
            );
            assert_eq!(first_net_id, second_net_id);
        }
        world.maintain();

        let entities: Vec<_> = world.entities().join().collect();
        assert_eq!(entities.len(), 1);
        assert_eq!(
            world.read_storage::<Client>().get(entities[0]).unwrap().id,
            cli_id
        );
    }
}
