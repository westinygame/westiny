use amethyst::{
    derive::SystemDesc,
    ecs::{System, SystemData, Read, Write},
    network::simulation::NetworkSimulationEvent,
    shrev::{ReaderId, EventChannel},
};

use anyhow::Result;
use std::net::SocketAddr;
use derive_new::new;

use westiny_common::{
    network::{PacketType, EntityState, NetworkEntityDelete, PlayerNotification, ShotEvent, PlayerUpdate},
    deserialize,
    events::AppEvent,
};
use westiny_common::network::PlayerDeath;

#[derive(SystemDesc, new)]
#[system_desc(name(NetworkMessageReceiverSystemDesc))]
pub struct NetworkMessageReceiverSystem {
    #[system_desc(event_channel_reader)]
    reader: ReaderId<NetworkSimulationEvent>,
}

impl<'s> System<'s> for NetworkMessageReceiverSystem {
    type SystemData = (
        Read<'s, EventChannel<NetworkSimulationEvent>>,
        Write<'s, EventChannel<AppEvent>>,
        Write<'s, EventChannel<Vec<EntityState>>>,
        Write<'s, EventChannel<PlayerUpdate>>,
        Write<'s, EventChannel<NetworkEntityDelete>>,
        Write<'s, EventChannel<PlayerNotification>>,
        Write<'s, EventChannel<ShotEvent>>,
        Write<'s, EventChannel<PlayerDeath>>,
    );

    fn run(&mut self, data: Self::SystemData) {
        let (net_event_ch,
        mut app_event,
        mut entity_state_update_channel,
        mut player_update_channel,
        mut entity_delete_channel,
        mut message_channel,
        mut shot_event_channel,
        mut death_event_channel) = data;
        for event in net_event_ch.read(&mut self.reader) {
            match event {
                NetworkSimulationEvent::Connect(addr) => log::debug!(
                    "Connect event from {:?}",
                    addr
                ),
                NetworkSimulationEvent::Disconnect(addr) => {
                    log::debug!(
                        "Disconnect event from {:?}",
                        addr
                    );

                    message_channel.single_write(PlayerNotification { message: "Server is unavailable!".to_string() });
                    app_event.single_write(AppEvent::Disconnect);
                },
                NetworkSimulationEvent::Message(addr, payload) => {
                    match self.process_payload(&addr,
                                               &payload,
                                               &mut entity_state_update_channel,
                                               &mut player_update_channel,
                                               &mut entity_delete_channel,
                                               &mut message_channel,
                                               &mut shot_event_channel,
                                               &mut death_event_channel) {
                        Ok(_) => log::debug!("Message from {} processed successfully.", addr),
                        Err(e) => {
                            log::error!("Could not process message! {:?}, payload: {:02x?}", e, payload)
                        }
                    }
                }
                _ => log::error!("Network error: {:?}", event),
            }
        }
    }
}

impl NetworkMessageReceiverSystem {
    fn process_payload(
        &self,
        addr: &SocketAddr,
        payload: &[u8],
        entity_update_channel: &mut EventChannel<Vec<EntityState>>,
        player_update_channel: &mut EventChannel<PlayerUpdate>,
        entity_delete_channel: &mut EventChannel<NetworkEntityDelete>,
        message_channel: &mut EventChannel<PlayerNotification>,
        shot_event_channel: &mut EventChannel<ShotEvent>,
        death_event_channel: &mut EventChannel<PlayerDeath>,
    ) -> Result<()> {

        log::debug!("Message: {:02x?}", payload);
        match deserialize(payload)? {
            PacketType::EntityStateUpdate(state) => {
                entity_update_channel.single_write(state);
                Ok(())
            }
            PacketType::EntityDelete(delete) => {
                log::debug!("Network entity delete, entity_id={:?}", delete.network_id);
                entity_delete_channel.single_write(delete);
                Ok(())
            }
            PacketType::PlayerUpdate(player_update) => {
                player_update_channel.single_write(player_update);
                Ok(())
            }
            PacketType::Notification(notification) => {
                message_channel.single_write(notification);
                Ok(())
            }
            PacketType::ShotEvent(shot) => {
                log::debug!("Shot event {:?}", shot);
                shot_event_channel.single_write(shot);
                Ok(())
            }
            PacketType::PlayerDeath(death) => {
                let notification = PlayerNotification { message: format!("{} died.", death.player_name) };
                message_channel.single_write(notification);
                death_event_channel.single_write(death);
                Ok(())
            }
            _ => Err(anyhow::anyhow!(
                "Unexpected message from {}, payload={:02x?}",
                addr,
                payload
            )),
        }
    }
}
