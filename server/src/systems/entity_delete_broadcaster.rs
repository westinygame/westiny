use derive_new::new;
use amethyst::ecs::{Read, ReadStorage, ReadExpect, Write, System, SystemData, Entities};
use amethyst::derive::SystemDesc;
use amethyst::shrev::{ReaderId, EventChannel};
use amethyst::network::simulation::{TransportResource, DeliveryRequirement, UrgencyRequirement};

use crate::resources::ClientRegistry;
use crate::components::NetworkId;
use westiny_common::resources::EntityDelete;
use westiny_common::{network, serialize};

// This system does two things:
//  - deletes the entities
//  - notifies the clients if the entity has NetworkId

#[derive(SystemDesc, new)]
#[system_desc(name(EntityDeleteBroadcasterSystemDesc))]
pub struct EntityDeleteBroadcasterSystem {
    #[system_desc(event_channel_reader)]
    reader: ReaderId<EntityDelete>,
}

impl<'s> System<'s> for EntityDeleteBroadcasterSystem {
    type SystemData = (
        Read<'s, EventChannel<EntityDelete>>,
        Entities<'s>,
        ReadExpect<'s, ClientRegistry>,
        Write<'s, TransportResource>,
        ReadStorage<'s, NetworkId>,
        );

    fn run(&mut self, (id_channel, entities, clients, mut net, network_ids): Self::SystemData) {
        for EntityDelete{entity_id} in id_channel.read(&mut self.reader) {

            if let Some(network_id) = network_ids.get(*entity_id) {
                send_to_clients(&clients, &mut net, network::NetworkEntityDelete{network_id: *network_id});
            }
            entities.delete(*entity_id).expect("Could not delete entity!");
        }
    }
}

fn send_to_clients(clients: &ClientRegistry, net: &mut TransportResource, network_entity_delete: network::NetworkEntityDelete)
{
    let message = serialize(&network::PacketType::EntityDelete(network_entity_delete))
        .expect("NetworkEntityDelete could not be serialized");

    clients.get_clients().iter().for_each(|&client|{
        net.send_with_requirements(client.addr, &message, DeliveryRequirement::Reliable, UrgencyRequirement::OnTick)
    }
    );
}
