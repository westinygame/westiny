use amethyst::{
    core::math::Point2,
    derive::SystemDesc,
    ecs::{
        Entities, Join, Read, ReadExpect, ReadStorage, System, SystemData, WriteExpect,
    },
    network::simulation::{DeliveryRequirement, TransportResource, UrgencyRequirement},
    shrev::{EventChannel, ReaderId},
};

use derive_new::new;

use westiny_common::{
    network::{ClientInitialData, PacketType},
    serialize,
    resources::{EntityDelete},
};

use crate::{
    components,
    components::EntityType,
    resources::{ClientID, ClientNetworkEvent, ClientRegistry, NetworkIdSupplier},
};
use westiny_common::resources::Seed;
use crate::systems::SpawnPlayerEvent;

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
        WriteExpect<'s, EventChannel<EntityDelete>>,
        WriteExpect<'s, TransportResource>,
        ReadExpect<'s, ClientRegistry>,
        ReadExpect<'s, Seed>,
        WriteExpect<'s, NetworkIdSupplier>,
        ReadStorage<'s, components::NetworkId>,
        ReadStorage<'s, components::Client>,
        WriteExpect<'s, EventChannel<SpawnPlayerEvent>>,
    );

    fn run(
        &mut self,
        (
            client_net_ec,
            entities,
            mut entity_delete_channel,
            mut net,
            client_registry,
            seed,
            mut net_id_supplier,
            network_ids,
            client,
            mut spawn_player_event_channel
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
                        let net_id = net_id_supplier.next(EntityType::Player);

                        if let Some((cli, net_id)) = (&client, &network_ids)
                            .join()
                            .find(|(&cli, _)| cli.id == *client_id)
                        {
                            log::info!(
                                "{:?} already connected, its entity already spawned: {:?}",
                                cli.id,
                                net_id
                            );
                        }

                        spawn_player_event_channel.single_write(SpawnPlayerEvent {
                            client: components::Client { id: *client_id },
                            network_id: net_id,
                        });

                        log::debug!(
                            "Player entity spawn requested for {}, {:?}, {:?}",
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
                            // TODO initial_pos should not be sent here. On the client side it will be processed from EntityStateUpdate messages anyway.
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
                    log::debug!("Removing disconnecting client's player entity [client_id: {:?}]", client_id);
                    Self::despawn_player(&entities, &mut entity_delete_channel, &client, client_id);
                }
            }
        }
    }
}

impl ClientIntroductionSystem {
    fn despawn_player(
        entities: &Entities<'_>,
        entity_delete_channel: &mut EventChannel<EntityDelete>,
        client_storage: &ReadStorage<'_, components::Client>,
        client_id: &ClientID,
    ) {
        for (entity, client) in (&*entities, client_storage).join() {
            if client.id() == client_id {
                entity_delete_channel.single_write(EntityDelete{entity_id: entity});
                return;
            }
        }
    }
}


