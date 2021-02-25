use amethyst::{
    derive::SystemDesc,
    ecs::{System, SystemData, Read, Write, ReadExpect, WriteStorage},
    network::simulation::NetworkSimulationEvent,
    shrev::{ReaderId, EventChannel},
    ui::{UiText},
};

use anyhow::Result;
use std::net::SocketAddr;
use derive_new::new;

use westiny_common::{
    network::{PacketType, EntityState, EntityHealth, NetworkEntityDelete},
    deserialize,
};
use crate::resources::{Hud};


#[derive(SystemDesc, new)]
#[system_desc(name(NetworkMessageReceiverSystemDesc))]
pub struct NetworkMessageReceiverSystem {
    #[system_desc(event_channel_reader)]
    reader: ReaderId<NetworkSimulationEvent>,
}

impl<'s> System<'s> for NetworkMessageReceiverSystem {
    type SystemData = (
        Read<'s, EventChannel<NetworkSimulationEvent>>,
        Write<'s, EventChannel<Vec<EntityState>>>,
        Write<'s, EventChannel<EntityHealth>>,
        Write<'s, EventChannel<NetworkEntityDelete>>,
        ReadExpect<'s, Hud>,
        WriteStorage<'s, UiText>,
    );

    fn run(&mut self, (net_event_ch, mut entity_state_update_channel, mut entity_health_channel, mut entity_delete_channel, hud, mut ui_texts): Self::SystemData) {
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

                    if let Some(messages) = ui_texts.get_mut(hud.messages) {
                        messages.text = "Server is unavailable!".to_string();
                    }
                },
                NetworkSimulationEvent::Message(addr, payload) => {
                    match self.process_payload(addr,
                                               payload,
                                               &mut entity_state_update_channel,
                                               &mut entity_health_channel,
                                               &mut entity_delete_channel) {
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
        entity_health_channel: &mut EventChannel<EntityHealth>,
        entity_delete_channel: &mut EventChannel<NetworkEntityDelete>,
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
            PacketType::EntityHealthUpdate(health_update) => {
                log::debug!("Network entity health change, entity_id={:?}, health={:?}", health_update.network_id, health_update.health);
                entity_health_channel.single_write(health_update);
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
