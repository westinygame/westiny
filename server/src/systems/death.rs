use crate::components::{Eliminated, Client};
use crate::resources::{ClientRegistry, StreamId};
use westiny_common::serialization::serialize;
use westiny_common::network::{PacketType, PlayerDeath};
use westiny_common::events::EntityDelete;
use westiny_common::metric_dimension::length::MeterVec2;
use blaminar::simulation::{TransportResource, DeliveryRequirement, UrgencyRequirement};
use bevy::prelude::{Query, With, EventWriter, Res, ResMut, Entity, Transform};

pub fn handle_death(eliminateds: Query<(Entity, &Transform, Option<&Client>), With<Eliminated>>,
                    client_registry: Res<ClientRegistry>,
                    mut net: ResMut<TransportResource>,
                    mut entity_delete: EventWriter<EntityDelete>) {
    for (entity, transform, maybe_client) in eliminateds.iter() {
        if let Some(client) = maybe_client {
            let player_name = client_registry.find_client(client.id).unwrap().player_name.clone();
            log::info!("{} died", player_name);

            // Dead player must be removed
            entity_delete.send(EntityDelete {entity_id: entity});

            let death_event_msg = serialize(&PacketType::PlayerDeath(
                    PlayerDeath {
                        player_name,
                        position: MeterVec2::from_pixel_vec(transform.translation.truncate())
                    }
            )).expect("Could not serialize PlayerDeath");

            client_registry.get_clients().iter().for_each(|&handle| {
                net.send_with_requirements(
                    handle.addr,
                    &death_event_msg,
                    DeliveryRequirement::ReliableSequenced(StreamId::PlayerDeath.into()),
                UrgencyRequirement::OnTick);
            });
        }
    }
}
