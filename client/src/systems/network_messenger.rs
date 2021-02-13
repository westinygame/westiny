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
    network::{PacketType, EntityState},
    deserialize,
};


#[derive(SystemDesc, new)]
#[system_desc(name(NetworkMessageReceiverSystemDesc))]
pub struct NetworkMessageReceiverSystem {
    #[system_desc(event_channel_reader)]
    reader: ReaderId<NetworkSimulationEvent>,
}

impl<'s> System<'s> for NetworkMessageReceiverSystem {
    type SystemData = (
        Read<'s, EventChannel<NetworkSimulationEvent>>,
        Write<'s, EventChannel<EntityState>>,
    );

    fn run(&mut self, (net_event_ch, mut entity_state_update_channel): Self::SystemData) {
        for event in net_event_ch.read(&mut self.reader) {
            match event {
                NetworkSimulationEvent::Connect(addr) => log::debug!(
                    "Connect event from {:?}",
                    addr
                ),
                NetworkSimulationEvent::Disconnect(addr) => log::debug!(
                    "Disconnect event from {:?}",
                    addr
                ),
                NetworkSimulationEvent::Message(addr, payload) => {
                    match self.process_payload(addr, payload, &mut entity_state_update_channel) {
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
        entity_update_channel: &mut EventChannel<EntityState>,
    ) -> Result<()> {

        log::debug!("Message: {:02x?}", payload);
        match deserialize(payload)? {
            PacketType::EntityStateUpdate(state) => {
                entity_update_channel.single_write(state);
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
