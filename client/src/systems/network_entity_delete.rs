use westiny_common::events::EntityDelete;
use westiny_common::network::NetworkEntityDelete;
use westiny_common::components::NetworkId;
use bevy::prelude::{Commands, EventReader, Entity, Query};

pub fn delete_entities(mut commands: Commands,
                   mut entity_delete_ec: EventReader<EntityDelete>,
                   mut net_entity_delete_ec: EventReader<NetworkEntityDelete>,
                   network_ids: Query<(Entity, &NetworkId)>) {
    let mut deletions: Vec<Entity> = entity_delete_ec.iter()
        .map(|del| del.entity_id)
        .collect();

    let net_deletions: Vec<NetworkId> = net_entity_delete_ec.iter()
        .map(|del| del.network_id)
        .collect();

    network_ids.iter()
        .filter(|(_, net_id)| net_deletions.contains(net_id))
        .for_each(|(entity, _)| deletions.push(entity));

    for entity_id in deletions.iter() {
        commands.entity(*entity_id).despawn();
    }

    // for deduplication -> bevy crashes if despawn is called on a nonexistent entity
    let mut deleted: Vec<Entity> = vec![];
    for entity_id in deletions.iter() {
        if !deleted.contains(entity_id) {
            commands.entity(*entity_id).despawn();
            deleted.push(*entity_id);
        }
    }
}
