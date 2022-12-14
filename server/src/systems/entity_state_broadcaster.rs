use crate::components;
use crate::resources::{ClientRegistry, StreamId};
use bevy::prelude::{Query, Res, ResMut, GlobalTransform};
use blaminar::simulation::{DeliveryRequirement, TransportResource, UrgencyRequirement};
use westiny_common::metric_dimension::length::{Meter, MeterVec2};
use westiny_common::{network, serialization::serialize};

/// This system is responsible for sending the transform of all the entities that has NetworkID
/// to every connected clients
pub fn broadcast_entity_state(
    client_registry: Res<ClientRegistry>,
    mut net: ResMut<TransportResource>,
    query: Query<(&components::NetworkId, &GlobalTransform)>,
) {
    let mut network_entities = Vec::new();
    for (network_id, transform) in query.iter() {
        let entity_state = network::EntityState {
            network_id: *network_id,
            position: MeterVec2 {
                x: Meter::from_pixel(transform.translation().x),
                y: Meter::from_pixel(transform.translation().y),
            },
            angle: transform.compute_transform().rotation.to_axis_angle().1,
        };
        network_entities.push(entity_state);
    }

    let msg = serialize(&network::PacketType::EntityStateUpdate(network_entities))
        .expect("entity state update could not be serialized");
    client_registry.get_clients().iter().for_each(|&handle| {
        net.send_with_requirements(
            handle.addr,
            &msg,
            DeliveryRequirement::UnreliableSequenced(StreamId::EntityStateUpdate.into()),
            UrgencyRequirement::OnTick,
        )
    })
}
