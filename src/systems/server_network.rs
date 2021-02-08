use amethyst::{
    derive::SystemDesc,
    ecs::{System, SystemData},
    network::simulation::{
        DeliveryRequirement, NetworkSimulationEvent, TransportResource, UrgencyRequirement,
    },
    shrev::ReaderId,
};

use amethyst::core::ecs::shrev::EventChannel;
use amethyst::core::ecs::{Read, Write, WriteExpect};
use anyhow::Result;
use bincode::{deserialize, serialize};
use std::net::SocketAddr;

use westiny_server::resources::ClientRegistry;
use westiny_common::network::{PacketType, ClientInitialData};

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

        log::info!("Message: {:?}", payload);
        match deserialize(payload)? {
            PacketType::ConnectionRequest { player_name } => {
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
                let response = serialize(&PacketType::ConnectionResponse(Ok(
                    ClientInitialData::new(),
                )))?;
                net.send_with_requirements(
                    *addr,
                    &response,
                    DeliveryRequirement::Reliable,
                    UrgencyRequirement::OnTick,
                );
                Ok(())
            },
            PacketType::InputState{ input } => {
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

#[cfg(test)]
mod test {
    use super::*;

    use amethyst::Error;
    use amethyst::prelude::*;
    use amethyst_test::prelude::*;
    use amethyst::network::simulation::Message;
    use westiny_common::network::{self, PacketType, ClientInitialData};

    #[test]
    fn send_response_on_connection_request() -> Result<(), Error>{
        amethyst::start_logger(Default::default());

        AmethystApplication::blank()
            .with_resource(TransportResource::default())
            .with_resource(ClientRegistry::new(1))
            .with_effect(|world| {
                let mut network_event_channel = world.fetch_mut::<EventChannel<NetworkSimulationEvent>>();
                network_event_channel.single_write(
                    NetworkSimulationEvent::Message(
                        socket_addr(),
                        bincode::serialize(&connection_request().clone()).unwrap().into()
                    )
                );
            })
            .with_system_desc(ServerNetworkSystemDesc::default(), "server_net_sys", &[])

            .with_assertion(|world| {
                let mut transport = world.write_resource::<TransportResource>();
                let messages = transport.drain_messages(|_| true);

                assert_eq!(messages.len(), 1, "Transport message queue contains {} messages", messages.len());

                let socket_address = socket_addr();
                let payload = serialize(&network::PacketType::ConnectionResponse(Ok(
                    network::ClientInitialData::new(),
                ))).unwrap();
                let expected_message = Message {
                    destination: socket_address,
                    payload: payload.into(),
                    delivery: DeliveryRequirement::Reliable,
                    urgency: UrgencyRequirement::OnTick
                };
                assert_eq!(messages[0], expected_message)
            })
            .run()
    }

    #[inline]
    fn socket_addr() -> SocketAddr {
        SocketAddr::from(([127, 0, 0, 1], 9999))
    }

    #[inline]
    fn connection_request() -> network::PacketType {
        network::PacketType::ConnectionRequest { player_name: "Clint Westwood".to_string() }
    }
}
