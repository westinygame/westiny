use westiny_common::events::EntityDelete;
use westiny_common::network::NetworkEntityDelete;
use westiny_common::components::NetworkId;
use bevy::prelude::{Commands, EventReader, Entity, Query}

fn delete_entities(mut commands: Commands,
                   entity_delete_ec: EventReader<EntityDelete>,
                   net_entity_delete_ec: EventReader<NetworkEntityDelete>,
                   network_ids: Query<(Entity, &NetworkId)>) {
    let mut deletions: Vec<Entity> = entity_delete_ec.iter()
        .map(|del| del.entity_id)
        .collect();

    let net_deletions: Vec<NetworkId> = net_entity_delete_ec.iter()
        .map(|del| del.network_id)
        .collect();

    network_ids.iter()
        .filter(|entity, net_id| net_deletions.contains(net_id))
        .for_each(|entity, _| deletions.push(entity));

    deletions.iter()
        // delete
        .map(|entity| entities.delete(*entity))
        // log errors
        .filter_map(|result| result.err())
        .for_each(|err| log::error!("Entity could not be deleted {}", err));
}
