use crate::components::NetworkId;
use crate::resources::ClientRegistry;
use bevy::prelude::{
    Commands, Entity, EventReader, ParallelSystemDescriptorCoercion, Query, Res,
    ResMut, SystemSet,
};
use blaminar::simulation::{DeliveryRequirement, TransportResource, UrgencyRequirement};
use westiny_common::events::EntityDelete;
use westiny_common::{network, serialization::serialize};

pub fn entity_delete_system_set() -> SystemSet {
    SystemSet::new()
        .label("entity_delete")
        .with_system(
            broadcast_net_id_deletion
                .label("entity_delete_broadcaster"),
        )
        .with_system(
            delete_entities
                .label("entity_delete")
                .after("entity_delete_broadcaster"),
        )
}

fn broadcast_net_id_deletion(
    mut entity_deletions: EventReader<EntityDelete>,
    network_ids: Query<&NetworkId>,
    clients: Res<ClientRegistry>,
    mut net: ResMut<TransportResource>,
) {
    for EntityDelete { entity_id: entity } in entity_deletions.iter() {
        if let Ok(&network_id) = network_ids.get(*entity) {
            let network_entity_delete = network::NetworkEntityDelete { network_id };
            let message = serialize(&network::PacketType::EntityDelete(network_entity_delete))
                .expect("NetworkEntityDelete could not be serialized");

            clients.get_clients().iter().for_each(|&client| {
                net.send_with_requirements(
                    client.addr,
                    &message,
                    DeliveryRequirement::Reliable,
                    UrgencyRequirement::OnTick,
                )
            });
        }
    }
}

fn delete_entities(mut commands: Commands, mut entity_deletions: EventReader<EntityDelete>) {
    // for deduplication -> bevy crashes if despawn is called on a nonexistent entity
    let mut deleted: Vec<Entity> = vec![];
    for del in entity_deletions.iter() {
        if !deleted.contains(&del.entity_id) {
            commands.entity(del.entity_id).despawn();
            deleted.push(del.entity_id);
        }
    }
}
