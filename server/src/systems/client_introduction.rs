use westiny_common::{
    network::{ClientInitialData, PacketType},
    serialization::serialize,
    //events::EntityDelete,
    resources::Seed
};

use crate::{
    components::{NetworkId, Client, EntityType},
    resources::{ClientID, ClientNetworkEvent, ClientRegistry, NetworkIdSupplier},
    systems::spawn::SpawnPlayerEvent
};
use bevy::prelude::{EventReader, ResMut, Res, Query, EventWriter};
use bevy::log::info;
use blaminar::simulation::{TransportResource, DeliveryRequirement, UrgencyRequirement};

pub fn introduce_new_clients(mut client_network_ec: EventReader<ClientNetworkEvent>,
                             //mut entity_delete_ec: EventWriter<EntityDelete>,
                             mut spawn_player_ec: EventWriter<SpawnPlayerEvent>,
                             mut transport: ResMut<TransportResource>,
                                 client_registry: Res<ClientRegistry>,
                                 seed: Res<Seed>,
                             mut network_id_supplier: ResMut<NetworkIdSupplier>,
                             mut query: Query<(&NetworkId, &Client)>) {
    // This vector is used for deduplicating ClientConnected events within one frame to avoid
    // multiple spawn for a single client.
    let mut added_clients = Vec::<(ClientID, NetworkId)>::new();

    for client_network_event in client_network_ec.iter() {
        match client_network_event {
            ClientNetworkEvent::ClientConnected(client_id) => {
                let client_handle = client_registry.find_client(*client_id).expect(&format!(
                    "Client [client_id: {:?}] not found in registry",
                    client_id
                ));

                let entity_network_id = if let Some((_, net_id)) =
                added_clients.iter().find(|(cli_id, _)| cli_id == client_id)
                {
                    info!(
                        "Player for {:?} already spawned: {:?}, not respawning.",
                        client_id,
                        net_id
                    );
                    *net_id
                } else {
                    let net_id = network_id_supplier.next(EntityType::Player);

                    for (net_id, client) in query.iter() {
                        if client.id == *client_id {
                            info!(
                                "{:?} already connected, its entity already spawned: {:?}",
                                client.id,
                                net_id
                            );
                        }
                    }

                    spawn_player_ec.send(SpawnPlayerEvent {
                        client: Client { id: *client_id },
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
                        seed: *seed
                    }));
                transport.send_with_requirements(
                    client_handle.addr,
                    &serialize(&connection_response).unwrap(),
                    DeliveryRequirement::Reliable,
                    UrgencyRequirement::OnTick,
                );

                /*
                broadcast_notification(
                    &mut net,
                    &client_registry,
                    PlayerNotification{message: format!("{} joined.", &client_handle.player_name)});
                */
            }
            ClientNetworkEvent::ClientDisconnected(client_id, player_name) => {
                log::debug!("Removing disconnecting client's player entity [client_id: {:?}]", client_id);
                /*
                Self::despawn_player(&entities, &mut entity_delete_channel, &client, client_id);

                broadcast_notification(
                    &mut net,
                    &client_registry,
                    PlayerNotification{message: format!("{} left the game.", &player_name)});
                */
            }
        }
    }
}

/*
fn broadcast_notification(
    net: &mut TransportResource,
    client_registry: &ClientRegistry,
    notification: PlayerNotification)
{
    let msg = serialize(&PacketType::Notification(notification)).expect("PlayerNotification could not be serialized");
    for &handle in client_registry.get_clients().iter() {
        net.send_with_requirements(handle.addr,
                                   &msg,
                                   DeliveryRequirement::Reliable,
                                   UrgencyRequirement::OnTick)
    }
}

fn despawn_player(
    mut query: Query<Entity, Client>,
    mut entity_delete_channel: EventWriter<EntityDelete>,
    client_id: &ClientID,
) {
    for (entity, client) in query.iter() {
        if client.id() == client_id {
            entity_delete_channel.single_write(EntityDelete{entity_id: entity});
            return;
        }
    }
}
*/

