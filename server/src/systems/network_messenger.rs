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
use crate::resources::{ClientRegistry, ClientNetworkEvent, NetworkCommand};


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
        Write<'s, EventChannel<NetworkCommand>>,
    );

    fn run(&mut self, (mut client_registry, net_event_ch, mut client_net_ec, mut command_channel): Self::SystemData) {
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
                    match self.process_payload(addr, payload, &mut client_registry, &mut client_net_ec, &mut command_channel) {
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
        command_channel: &mut EventChannel<NetworkCommand>,
    ) -> Result<()> {

        log::debug!("Message: {:02x?}", payload);
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
                registry
                    .find_by_addr(addr)
                    .map(|handle| command_channel.single_write(NetworkCommand::Input { id: handle.id, input }))
                    .ok_or(anyhow::anyhow!("Valid input command from unregistered client! Address: {:?}", addr))
            },
            _ => Err(anyhow::anyhow!(
                "Unexpected message from {}, payload={:02x?}",
                addr,
                payload
            )),
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    use amethyst::{Error, StateEventReader, core::math::Point2};
    use amethyst::prelude::*;
    use amethyst_test::prelude::*;
    use westiny_common::{network, components::Input};

    fn create_testapp() -> AmethystApplication<GameData<'static, 'static>, StateEvent, StateEventReader>
    {
        amethyst::start_logger(Default::default());
        AmethystApplication::blank()
            .with_resource(EventChannel::<ClientNetworkEvent>::new())
            .with_resource(EventChannel::<NetworkCommand>::new())
            .with_setup(move |world: &mut World| {
                let client_net_channel = world.fetch_mut::<EventChannel<ClientNetworkEvent>>().register_reader();
                world.insert(client_net_channel);

                let command_channel = world.fetch_mut::<EventChannel<NetworkCommand>>().register_reader();
                world.insert(command_channel);
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
                let mut reader_id = world.write_resource::<ReaderId<ClientNetworkEvent>>();

                let events: Vec<&ClientNetworkEvent> = client_net_ec.read(&mut reader_id).collect();
                assert_eq!(1, events.len(), "There should be exactly 1 ClientNetworkEvent on channel");
                assert!(matches!(events[0], ClientNetworkEvent::ClientConnected(_)));
            })
    }


    #[test]
    fn receiver_registers_client_on_connection_request() -> Result<(), Error> {
        create_testapp()
            .with_effect(|world| {
                let mut network_event_channel = world.fetch_mut::<EventChannel<NetworkSimulationEvent>>();
                network_event_channel.single_write(NetworkSimulationEvent::Disconnect(socket_addr()));
            })
            .with_assertion(|world| {
                let client_net_ec = world.fetch_mut::<EventChannel<ClientNetworkEvent>>();
                let mut reader_id = world.write_resource::<ReaderId<ClientNetworkEvent>>();

                let events: Vec<&ClientNetworkEvent> = client_net_ec.read(&mut reader_id).collect();
                assert_eq!(1, events.len(), "There should be exactly 1 ClientNetworkEvent on channel");
                assert!(matches!(events[0], ClientNetworkEvent::ClientDisconnected(_)));
            })
            .run()
    }

    fn make_input() -> Input {
        let mut inp = Input::default();
        inp.forward = true;
        inp.cursor = Point2::new(42.0, 99.99);
        inp
    }

    #[test]
    fn client_input_should_be_forwarded() -> Result<(), Error> {
        create_testapp()
            .with_effect(|world| {
                let mut network_event_channel = world.fetch_mut::<EventChannel<NetworkSimulationEvent>>();
                network_event_channel.single_write(
                    NetworkSimulationEvent::Message(
                        socket_addr(),
                        bincode::serialize(&PacketType::InputState { input: make_input() }).unwrap().into()
                    )
                );
            })
            .with_assertion(|world| {
                let registry = world.read_resource::<ClientRegistry>();
                let handle = registry.find_by_addr(&socket_addr()).expect("Client is not registered yet!?");

                let command_channel = world.fetch_mut::<EventChannel<NetworkCommand>>();
                let mut reader_id = world.write_resource::<ReaderId<NetworkCommand>>();

                let commands: Vec<&NetworkCommand> = command_channel.read(&mut reader_id).collect();
                assert_eq!(1, commands.len());
                assert!(matches!(commands[0], NetworkCommand::Input { id, input } if input == &make_input() && &handle.id == id ));
            })
            .run()
    }

    #[test]
    fn not_connected_client_input_should_not_be_forwarded() -> Result<(), Error> {
        create_testapp()
            .with_effect(|world| {
                let mut network_event_channel = world.fetch_mut::<EventChannel<NetworkSimulationEvent>>();
                network_event_channel.single_write(
                    NetworkSimulationEvent::Message(
                        SocketAddr::from(([1,2,3,4], 55555)),
                        bincode::serialize(&PacketType::InputState { input: make_input() }).unwrap().into()
                    )
                );
            })
            .with_assertion(|world| {
                let command_channel = world.fetch_mut::<EventChannel<NetworkCommand>>();
                let mut reader_id = world.write_resource::<ReaderId<NetworkCommand>>();

                let commands: Vec<&NetworkCommand> = command_channel.read(&mut reader_id).collect();
                assert_eq!(0, commands.len(), "Command channel should be empty, but it has: {:?}", commands[0]);
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
