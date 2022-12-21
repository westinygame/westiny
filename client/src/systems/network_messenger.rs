use crate::NetworkSimulationEvent;
use anyhow::Result;
use bevy::prelude::*;
use std::net::SocketAddr;
use westiny_common::network::{
    EntityState, NetworkEntityDelete, PacketType, PlayerNotification, PlayerUpdate, ShotEvent,
};
use westiny_common::{network::PlayerDeath, serialization::deserialize};

pub fn receive_network_messages(
    mut network_event: EventReader<NetworkSimulationEvent>,
    // mut app_event: EventWriter<AppEvent>,
    mut entity_states: EventWriter<Vec<EntityState>>,
    mut player_update: EventWriter<PlayerUpdate>,
    mut entity_delete: EventWriter<NetworkEntityDelete>,
    mut notification: EventWriter<PlayerNotification>,
    mut shot: EventWriter<ShotEvent>,
    mut player_death: EventWriter<PlayerDeath>,
) {
    for event in network_event.iter() {
        match event {
            NetworkSimulationEvent::Connect(addr) => log::debug!("Connect event from {:?}", addr),
            NetworkSimulationEvent::Disconnect(addr) => {
                log::debug!("Disconnect event from {:?}", addr);

                notification.send(PlayerNotification {
                    message: "Server is unavailable!".to_string(),
                });
                //app_event.send(AppEvent::Disconnect);
            }
            NetworkSimulationEvent::Message(addr, payload) => {
                match process_payload(
                    addr,
                    payload,
                    &mut entity_states,
                    &mut player_update,
                    &mut entity_delete,
                    &mut notification,
                    &mut shot,
                    &mut player_death,
                ) {
                    Ok(_) => log::debug!("Message from {} processed successfully.", addr),
                    Err(e) => {
                        log::error!(
                            "Could not process message! {:?}, payload: {:02x?}",
                            e,
                            payload
                        )
                    }
                }
            }
            _ => log::error!("Network error: {:?}", event),
        }
    }
}

#[allow(clippy::too_many_arguments)]
fn process_payload(
    addr: &SocketAddr,
    payload: &[u8],
    entity_update_channel: &mut EventWriter<Vec<EntityState>>,
    player_update_channel: &mut EventWriter<PlayerUpdate>,
    entity_delete_channel: &mut EventWriter<NetworkEntityDelete>,
    message_channel: &mut EventWriter<PlayerNotification>,
    shot_event_channel: &mut EventWriter<ShotEvent>,
    death_event_channel: &mut EventWriter<PlayerDeath>,
) -> Result<()> {
    log::debug!("Message: {:02x?}", payload);
    match deserialize(payload)? {
        PacketType::EntityStateUpdate(state) => {
            log::debug!("Entity State update, state={:?}", state);
            entity_update_channel.send(state);
            Ok(())
        }
        PacketType::EntityDelete(delete) => {
            log::debug!("Network entity delete, entity_id={:?}", delete.network_id);
            entity_delete_channel.send(delete);
            Ok(())
        }
        PacketType::PlayerUpdate(player_update) => {
            log::debug!("Player update, {:?}", player_update);
            player_update_channel.send(player_update);
            Ok(())
        }
        PacketType::Notification(notification) => {
            log::debug!("Notification, {:?}", notification);
            message_channel.send(notification);
            Ok(())
        }
        PacketType::ShotEvent(shot) => {
            log::debug!("Shot event {:?}", shot);
            shot_event_channel.send(shot);
            Ok(())
        }
        PacketType::PlayerDeath(death) => {
            let notification = PlayerNotification {
                message: format!("{} died.", death.player_name),
            };
            message_channel.send(notification);
            death_event_channel.send(death);
            Ok(())
        }
        _ => Err(anyhow::anyhow!(
            "Unexpected message from {}, payload={:02x?}",
            addr,
            payload
        )),
    }
}
