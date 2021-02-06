use amethyst::{
    derive::SystemDesc,
    ecs::{System, SystemData},
    network::simulation::{
        DeliveryRequirement, NetworkSimulationEvent, TransportResource, UrgencyRequirement,
    },
    shrev::ReaderId,
};

use crate::network;
use amethyst::core::ecs::shrev::EventChannel;
use amethyst::core::ecs::{Read, Write, WriteExpect};
use amethyst::core::math::Point2;
use anyhow::Result;
use bincode::{deserialize, serialize};
use std::net::SocketAddr;

use westiny_server::resources::ClientRegistry;

#[derive(SystemDesc)]
#[system_desc(name(ServerNetworkSystemDesc))]
pub struct ServerNetworkSystem {
    #[system_desc(event_channel_reader)]
    reader: ReaderId<NetworkSimulationEvent>,
}

impl ServerNetworkSystem {
    pub fn new(reader: ReaderId<NetworkSimulationEvent>) -> Self {
        Self { reader }
    }
}

impl<'s> System<'s> for ServerNetworkSystem {
    type SystemData = (
        Write<'s, TransportResource>,
        WriteExpect<'s, ClientRegistry>,
        Read<'s, EventChannel<NetworkSimulationEvent>>,
    );

    fn run(&mut self, (mut net, mut client_registry, net_event_ch): Self::SystemData) {
        for event in net_event_ch.read(&mut self.reader) {
            match event {
                NetworkSimulationEvent::Connect(addr) => log::info!(
                    "Client connection from {:?}, expecting initial message",
                    addr
                ),
                NetworkSimulationEvent::Disconnect(addr) => {
                    if let Err(e) = self.disconnect_client(addr, &mut client_registry) {
                        log::error!("Error during disconnect_client: {}", e);
                    }
                }
                NetworkSimulationEvent::Message(addr, payload) => {
                    match self.process_payload(addr, payload, &mut net, &mut client_registry) {
                        Ok(_) => log::debug!("Message from {} processed successfully.", addr),
                        Err(e) => {
                            log::error!("Could not process message! {}, payload: {:?}", e, payload)
                        }
                    }
                }
                _ => log::error!("Network error: {:?}", event),
            }
        }
    }
}

impl ServerNetworkSystem {
    fn disconnect_client(
        &mut self,
        addr: &SocketAddr,
        registry: &mut ClientRegistry,
    ) -> Result<()> {
        log::info!("Disconnecting {:?}", addr);
        registry.remove(addr)?;
        Ok(())
    }

    fn process_payload(
        &mut self,
        addr: &SocketAddr,
        payload: &[u8],
        net: &mut TransportResource,
        registry: &mut ClientRegistry,
    ) -> Result<()> {
        use network::*;

        log::info!("Message: {:?}", payload);
        match deserialize(payload)? {
            PackageType::ConnectionRequest { player_name } => {
                log::debug!("Connection request received: {}, {}", addr, player_name);
                // TODO response errors from registry
                let client_id = registry.add(addr, player_name.as_str())?;
                log::info!(
                    "Client from {} as player {} connection request accepted. ClientID={:?}",
                    addr,
                    player_name,
                    client_id
                );
                // TODO load last position or generate brand new
                let response = serialize(&PackageType::ConnectionResponse(Ok(
                    ClientInitialData::new(Point2::new(0.0, 0.0)),
                )))?;
                net.send_with_requirements(
                    *addr,
                    &response,
                    DeliveryRequirement::Reliable,
                    UrgencyRequirement::OnTick,
                );
                Ok(())
            },
            PackageType::InputState{ input } => {
                log::info!("Input state received: {:?} ", input);
                Ok(())
            },
            _ => Err(anyhow::anyhow!(
                "Unexpected message from {}, {:?}",
                addr,
                payload
            )),
        }
    }
}
