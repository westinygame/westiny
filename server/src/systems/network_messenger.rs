use anyhow::Result;
use std::net::SocketAddr;
use bevy::log::prelude::*;

use westiny_common::{
    network::{PacketType},
    serialization::deserialize
};

use crate::resources::{ClientRegistry, ClientNetworkEvent, NetworkCommand};
use bevy::ecs::prelude::ResMut;
use bevy::prelude::{EventReader, EventWriter};
use blaminar::simulation::NetworkSimulationEvent;

pub fn read_network_messages(mut client_registry:    ResMut<ClientRegistry>,
                             mut network_sim_ec:     EventReader<NetworkSimulationEvent>,
                             mut client_network_ec:  EventWriter<ClientNetworkEvent>,
                             mut network_command_ec: EventWriter<NetworkCommand>) {
    for event in network_sim_ec.iter() {
        match event {
            NetworkSimulationEvent::Connect(addr) => info!(
                "Client connection from {:?}, expecting initial message",
                addr
            ),
            NetworkSimulationEvent::Disconnect(addr) => {
                if let Err(e) = disconnect_client(addr, &mut client_registry, &mut client_network_ec) {
                    error!("Error during disconnect_client: {}", e);
                }
            }
            NetworkSimulationEvent::Message(addr, payload) => {
                match process_payload(addr, payload, &mut client_registry, &mut client_network_ec, &mut network_command_ec) {
                    Ok(_) => debug!("Message from {} processed successfully.", addr),
                    Err(e) => {
                        error!("Could not process message! {}, payload: {:?}", e, payload)
                    }
                }
            }
            _ => error!("Network error: {:?}", event),
        }
    }
}
    fn disconnect_client(addr: &SocketAddr,
                         registry: &mut ClientRegistry,
                         client_event_channel: &mut EventWriter<ClientNetworkEvent>,
    ) -> Result<()> {
        info!("Disconnecting {:?}", addr);
        let handle = registry.find_by_addr(&addr).ok_or(anyhow::anyhow!("Could not find address {} in registry", addr))?;
        let player_name = handle.player_name.clone();
        let id = registry.remove(addr)?;
        client_event_channel.send(ClientNetworkEvent::ClientDisconnected(id, player_name));
        Ok(())
    }

    fn process_payload(
        addr: &SocketAddr,
        payload: &[u8],
        registry: &mut ClientRegistry,
        client_net_event_channel: &mut EventWriter<ClientNetworkEvent>,
        command_channel: &mut EventWriter<NetworkCommand>,
    ) -> Result<()> {

        debug!("Message: {:02x?}", payload);
        match deserialize(payload)? {
            PacketType::ConnectionRequest { player_name } => {
                debug!("Connection request received: {}, {}", addr, player_name);
                // TODO response errors from registry
                let client_id = registry.add(addr, player_name.as_str())?;
                info!(
                    "Client from {} as player {} connection request accepted. ClientID={:?}",
                    addr,
                    player_name,
                    client_id
                );

                client_net_event_channel.send(ClientNetworkEvent::ClientConnected(client_id));
                Ok(())
            },
            PacketType::InputState{ input } => {
                registry
                    .find_by_addr(addr)
                    .map(|handle| command_channel.send(NetworkCommand::Input { id: handle.id, input }))
                    .ok_or(anyhow::anyhow!("Valid input command from unregistered client! Address: {:?}", addr))
            },
            _ => Err(anyhow::anyhow!(
                "Unexpected message from {}, payload={:02x?}",
                addr,
                payload
            )),
        }
    }

// #[cfg(test)]
// mod test {
//     use super::*;
//
//     use amethyst::{Error, StateEventReader, core::math::Point2};
//     use amethyst::prelude::*;
//     use amethyst_test::prelude::*;
//     use westiny_common::{network, components::{InputFlags, Input}, serialize};
//     use westiny_common::metric_dimension::length::Meter;
//
//     fn create_testapp() -> AmethystApplication<GameData<'static, 'static>, StateEvent, StateEventReader>
//     {
//         amethyst::start_logger(Default::default());
//         AmethystApplication::blank()
//             .with_resource(EventChannel::<ClientNetworkEvent>::new())
//             .with_resource(EventChannel::<NetworkCommand>::new())
//             .with_setup(move |world: &mut World| {
//                 let client_net_channel = world.fetch_mut::<EventChannel<ClientNetworkEvent>>().register_reader();
//                 world.insert(client_net_channel);
//
//                 let command_channel = world.fetch_mut::<EventChannel<NetworkCommand>>().register_reader();
//                 world.insert(command_channel);
//             })
//             .with_resource(ClientRegistry::new(1))
//             .with_effect(|world| {
//                 let mut network_event_channel = world.fetch_mut::<EventChannel<NetworkSimulationEvent>>();
//                 let req = connection_request();
//                 network_event_channel.single_write(
//                     NetworkSimulationEvent::Message(
//                         socket_addr(),
//                         serialize(&req).unwrap().into()
//                     )
//                 );
//             })
//             .with_system_desc(NetworkMessageReceiverSystemDesc::default(), "receiver", &[])
//             .with_assertion(|world: &mut World| {
//                 let client_net_ec = world.fetch_mut::<EventChannel<ClientNetworkEvent>>();
//                 let mut reader_id = world.write_resource::<ReaderId<ClientNetworkEvent>>();
//
//                 let events: Vec<&ClientNetworkEvent> = client_net_ec.read(&mut reader_id).collect();
//                 assert_eq!(1, events.len(), "There should be exactly 1 ClientNetworkEvent on channel");
//                 assert!(matches!(events[0], ClientNetworkEvent::ClientConnected(_)));
//             })
//     }
//
//
//     #[test]
//     fn receiver_registers_client_on_connection_request() -> Result<(), Error> {
//         create_testapp()
//             .with_effect(|world| {
//                 let mut network_event_channel = world.fetch_mut::<EventChannel<NetworkSimulationEvent>>();
//                 network_event_channel.single_write(NetworkSimulationEvent::Disconnect(socket_addr()));
//             })
//             .with_assertion(|world| {
//                 let client_net_ec = world.fetch_mut::<EventChannel<ClientNetworkEvent>>();
//                 let mut reader_id = world.write_resource::<ReaderId<ClientNetworkEvent>>();
//
//                 let events: Vec<&ClientNetworkEvent> = client_net_ec.read(&mut reader_id).collect();
//                 assert_eq!(1, events.len(), "There should be exactly 1 ClientNetworkEvent on channel");
//                 assert!(matches!(events[0], ClientNetworkEvent::ClientDisconnected(_, _)));
//             })
//             .run()
//     }
//
//     fn make_input() -> Input {
//         let mut inp = Input::default();
//         inp.flags |= InputFlags::FORWARD;
//         inp.cursor = Point2::new(Meter::from_pixel(42.0), Meter::from_pixel(99.99));
//         inp
//     }
//
//     #[test]
//     fn client_input_should_be_forwarded() -> Result<(), Error> {
//         create_testapp()
//             .with_effect(|world| {
//                 let mut network_event_channel = world.fetch_mut::<EventChannel<NetworkSimulationEvent>>();
//                 network_event_channel.single_write(
//                     NetworkSimulationEvent::Message(
//                         socket_addr(),
//                         serialize(&PacketType::InputState { input: make_input() }).unwrap().into()
//                     )
//                 );
//             })
//             .with_assertion(|world| {
//                 let registry = world.read_resource::<ClientRegistry>();
//                 let handle = registry.find_by_addr(&socket_addr()).expect("Client is not registered yet!?");
//
//                 let command_channel = world.fetch_mut::<EventChannel<NetworkCommand>>();
//                 let mut reader_id = world.write_resource::<ReaderId<NetworkCommand>>();
//
//                 let commands: Vec<&NetworkCommand> = command_channel.read(&mut reader_id).collect();
//                 assert_eq!(1, commands.len());
//                 assert!(matches!(commands[0], NetworkCommand::Input { id, input } if input == &make_input() && &handle.id == id ));
//             })
//             .run()
//     }
//
//     #[test]
//     fn not_connected_client_input_should_not_be_forwarded() -> Result<(), Error> {
//         create_testapp()
//             .with_effect(|world| {
//                 let mut network_event_channel = world.fetch_mut::<EventChannel<NetworkSimulationEvent>>();
//                 network_event_channel.single_write(
//                     NetworkSimulationEvent::Message(
//                         SocketAddr::from(([1,2,3,4], 55555)),
//                         serialize(&PacketType::InputState { input: make_input() }).unwrap().into()
//                     )
//                 );
//             })
//             .with_assertion(|world| {
//                 let command_channel = world.fetch_mut::<EventChannel<NetworkCommand>>();
//                 let mut reader_id = world.write_resource::<ReaderId<NetworkCommand>>();
//
//                 let commands: Vec<&NetworkCommand> = command_channel.read(&mut reader_id).collect();
//                 assert_eq!(0, commands.len(), "Command channel should be empty, but it has: {:?}", commands[0]);
//             })
//             .run()
//     }
//
//     #[inline]
//     fn socket_addr() -> SocketAddr {
//         SocketAddr::from(([127, 0, 0, 1], 9999))
//     }
//
//     #[inline]
//     fn connection_request() -> network::PacketType {
//         network::PacketType::ConnectionRequest { player_name: "Clint Westwood".to_string() }
//     }
// }
