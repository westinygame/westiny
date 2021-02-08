use amethyst::{
    derive::SystemDesc,
    ecs::{System, SystemData, Read, Write, WriteExpect},
    network::simulation::NetworkSimulationEvent,
    shrev::{ReaderId, EventChannel},
};

use anyhow::Result;
use bincode::deserialize;
use std::net::SocketAddr;
use derive_new::new;

use westiny_common::network::PacketType;
use crate::resources::{ClientRegistry, ClientNetworkEvent};


#[derive(SystemDesc, new)]
#[system_desc(name(NetworkMessageReceiverSystemDesc))]
pub struct NetworkMessageReceiverSystem {
    #[system_desc(event_channel_reader)]
    reader: ReaderId<NetworkSimulationEvent>,
}

impl<'s> System<'s> for NetworkMessageReceiverSystem {
    type SystemData = (
        WriteExpect<'s, ClientRegistry>,
        Read<'s, EventChannel<NetworkSimulationEvent>>,
        Write<'s, EventChannel<ClientNetworkEvent>>,
    );

    fn run(&mut self, (mut client_registry, net_event_ch, mut client_net_ec): Self::SystemData) {
        for event in net_event_ch.read(&mut self.reader) {
            match event {
                NetworkSimulationEvent::Connect(addr) => log::info!(
                    "Client connection from {:?}, expecting initial message",
                    addr
                ),
                NetworkSimulationEvent::Disconnect(addr) => {
                    if let Err(e) = self.disconnect_client(addr, &mut client_registry, &mut client_net_ec) {
                        log::error!("Error during disconnect_client: {}", e);
                    }
                }
                NetworkSimulationEvent::Message(addr, payload) => {
                    match self.process_payload(addr, payload, &mut client_registry, &mut client_net_ec) {
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

impl NetworkMessageReceiverSystem {
    fn disconnect_client(
        &self,
        addr: &SocketAddr,
        registry: &mut ClientRegistry,
        client_event_channel: &mut EventChannel<ClientNetworkEvent>,
    ) -> Result<()> {
        log::info!("Disconnecting {:?}", addr);
        let id = registry.remove(addr)?;
        client_event_channel.single_write(ClientNetworkEvent::ClientDisconnected(id));
        Ok(())
    }

    fn process_payload(
        &self,
        addr: &SocketAddr,
        payload: &[u8],
        registry: &mut ClientRegistry,
        client_net_event_channel: &mut EventChannel<ClientNetworkEvent>,
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

                client_net_event_channel.single_write(ClientNetworkEvent::ClientConnected(client_id));
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
    use westiny_common::network;

    type ClientNetworkEventReaderResource = Option<ReaderId<ClientNetworkEvent>>;
    #[test]
    fn receiver_registers_client_on_connection_request() -> Result<(), Error> {
        amethyst::start_logger(Default::default());

        AmethystApplication::blank()
            .with_resource(EventChannel::<ClientNetworkEvent>::new())
            .with_resource(ClientNetworkEventReaderResource::None)
            .with_setup(move |world: &mut World| {
                let mut reader_id = world.fetch_mut::<ClientNetworkEventReaderResource>();
                *reader_id = Some(world.fetch_mut::<EventChannel<ClientNetworkEvent>>().register_reader());
            })
            .with_resource(ClientRegistry::new(1))
            .with_effect(|world| {
                let mut network_event_channel = world.fetch_mut::<EventChannel<NetworkSimulationEvent>>();
                let req = connection_request();
                network_event_channel.single_write(
                    NetworkSimulationEvent::Message(
                        socket_addr(),
                        bincode::serialize(&req).unwrap().into()
                    )
                );
            })
            .with_system_desc(NetworkMessageReceiverSystemDesc::default(), "receiver", &[])
            .with_assertion(|world: &mut World| {
                let client_net_ec = world.fetch_mut::<EventChannel<ClientNetworkEvent>>();
                let mut reader_id = world.write_resource::<ClientNetworkEventReaderResource>();

                let events: Vec<&ClientNetworkEvent> = client_net_ec.read(reader_id.as_mut().unwrap()).collect();
                assert_eq!(1, events.len(), "There should be exactly 1 ClientNetworkEvent on channel");
                if let ClientNetworkEvent::ClientConnected(_) = events[0] {
                    log::info!("Connect event confirmed")
                } else {
                    panic!("There's no connect event on event channel")
                }
            })
            .with_effect(|world| {
                let mut network_event_channel = world.fetch_mut::<EventChannel<NetworkSimulationEvent>>();
                network_event_channel.single_write(NetworkSimulationEvent::Disconnect(socket_addr()));
            })
            .with_assertion(|world| {
                let client_net_ec = world.fetch_mut::<EventChannel<ClientNetworkEvent>>();
                let mut reader_id = world.write_resource::<ClientNetworkEventReaderResource>();

                let events: Vec<&ClientNetworkEvent> = client_net_ec.read(reader_id.as_mut().unwrap()).collect();
                assert_eq!(1, events.len(), "There should be exactly 1 ClientNetworkEvent on channel");
                if let ClientNetworkEvent::ClientDisconnected(_) = events[0] {
                    log::info!("Disconnect event confirmed");
                } else {
                    panic!("There's no disconnect event on event channel")
                }
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