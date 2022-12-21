use anyhow::Result;
use std::net::SocketAddr;

use westiny_common::{network::PacketType, serialization::deserialize};

use crate::resources::{ClientNetworkEvent, ClientRegistry, NetworkCommand};
use bevy::prelude::{EventReader, EventWriter, ResMut};
use blaminar::simulation::NetworkSimulationEvent;

pub fn read_network_messages(
    mut client_registry: ResMut<ClientRegistry>,
    mut network_sim_ec: EventReader<NetworkSimulationEvent>,
    mut client_network_ec: EventWriter<ClientNetworkEvent>,
    mut network_command_ec: EventWriter<NetworkCommand>,
) {
    for event in network_sim_ec.iter() {
        match event {
            NetworkSimulationEvent::Connect(addr) => log::info!(
                "Client connection from {:?}, expecting initial message",
                addr
            ),
            NetworkSimulationEvent::Disconnect(addr) => {
                if let Err(e) =
                    disconnect_client(addr, &mut client_registry, &mut client_network_ec)
                {
                    log::error!("Error during disconnect_client: {}", e);
                }
            }
            NetworkSimulationEvent::Message(addr, payload) => {
                match process_payload(
                    addr,
                    payload,
                    &mut client_registry,
                    &mut client_network_ec,
                    &mut network_command_ec,
                ) {
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
fn disconnect_client(
    addr: &SocketAddr,
    registry: &mut ClientRegistry,
    client_event_channel: &mut EventWriter<ClientNetworkEvent>,
) -> Result<()> {
    log::info!("Disconnecting {:?}", addr);
    let handle = registry
        .find_by_addr(addr)
        .ok_or_else(|| anyhow::anyhow!("Could not find address {} in registry", addr))?;
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

            client_net_event_channel.send(ClientNetworkEvent::ClientConnected(client_id));
            Ok(())
        }
        PacketType::InputState { input } => registry
            .find_by_addr(addr)
            .map(|handle| {
                command_channel.send(NetworkCommand::Input {
                    id: handle.id,
                    input,
                })
            })
            .ok_or_else(|| {
                anyhow::anyhow!(
                    "Valid input command from unregistered client! Address: {:?}",
                    addr
                )
            }),
        _ => Err(anyhow::anyhow!(
            "Unexpected message from {}, payload={:02x?}",
            addr,
            payload
        )),
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::resources;
    use crate::resources::ClientID;
    use bevy::ecs::schedule::{IntoSystemDescriptor, SystemDescriptor};
    use bevy::prelude::*;
    use std::net::{IpAddr, SocketAddr};
    use w_bevy_test::{assertion, TestApp};
    use westiny_common::PlayerName;

    fn make_socket_addr(ip: &str, port: u16) -> SocketAddr {
        use std::str::FromStr;
        SocketAddr::new(IpAddr::from_str(ip).unwrap(), port)
    }

    struct TestAppParams {
        client_registry_capacity: usize,
        preloaded_clients: Vec<(SocketAddr, String)>,
        send_event: NetworkSimulationEvent,
    }

    fn make_testapp(params: TestAppParams) -> App {
        let mut client_registry = ClientRegistry::new(params.client_registry_capacity);
        for (addr, name) in params.preloaded_clients {
            client_registry.add(&addr, &name).unwrap();
        }

        let mut appl = App::new();
        appl.add_event::<ClientNetworkEvent>()
            .add_event::<NetworkCommand>()
            .insert_resource(client_registry)
            .insert_resource(resources::NetworkIdSupplier::new())
            .add_system(read_network_messages)
            .send_events(vec![Some(params.send_event)]);
        appl
    }

    fn assert_client_in_registry(
        client_addr: SocketAddr,
        expected_in_registry: bool,
    ) -> SystemDescriptor {
        (move |registry: Res<ClientRegistry>| {
            assert_eq!(
                registry.find_by_addr(&client_addr).is_some(),
                expected_in_registry,
                "Client {} is{} expected to be in registry",
                &client_addr,
                if expected_in_registry { "" } else { " not" }
            )
        })
        .into_descriptor()
    }

    #[test]
    fn test_disconnect_client_happy_path() {
        let disconnecting_addr = make_socket_addr("127.0.0.1", 1111);

        let params = TestAppParams {
            client_registry_capacity: 2,
            preloaded_clients: vec![
                (make_socket_addr("127.0.0.1", 3333), "egyik".to_string()),
                (disconnecting_addr.clone(), "masik".to_string()),
            ],
            send_event: NetworkSimulationEvent::Disconnect(disconnecting_addr),
        };

        make_testapp(params)
            .add_assert_system(assertion::assert_event_count::<ClientNetworkEvent>(1))
            // ClientDisconnected event sent
            .add_assert_system(assertion::assert_event(
                ClientNetworkEvent::ClientDisconnected(
                    ClientID(1),
                    PlayerName("masik".to_string()),
                ),
            ))
            // Disconnected client removed from registry
            .add_assert_system(assert_client_in_registry(disconnecting_addr, false))
            .run();
    }

    fn connection_request_event(requesting_addr: SocketAddr) -> NetworkSimulationEvent {
        let payload = westiny_common::serialization::serialize(&PacketType::ConnectionRequest {
            player_name: "Westwood".to_string(),
        })
        .unwrap();
        NetworkSimulationEvent::Message(requesting_addr, blaminar::Bytes::from(payload))
    }

    #[test]
    fn connection_request_client_registered() {
        let connecting_addr = make_socket_addr("0.1.2.3", 1234);

        let params = TestAppParams {
            client_registry_capacity: 2,
            preloaded_clients: vec![(make_socket_addr("192.168.0.15", 3333), "egyik".to_string())],
            send_event: connection_request_event(connecting_addr.clone()),
        };

        make_testapp(params)
            // assert client is in registry
            .add_assert_system(assert_client_in_registry(connecting_addr, true))
            .add_assert_system(assertion::assert_event(
                ClientNetworkEvent::ClientConnected(ClientID(1)),
            ))
            .run();
    }

    #[test]
    fn connection_request_registry_full() {
        let connecting_addr = make_socket_addr("0.1.2.3", 1234);

        let params = TestAppParams {
            client_registry_capacity: 0,
            preloaded_clients: vec![],
            send_event: connection_request_event(connecting_addr),
        };
        make_testapp(params)
            .add_assert_system(assert_client_in_registry(connecting_addr, false))
            .run();
    }

    #[test]
    fn connection_request_already_registered() {
        let connecting_addr = make_socket_addr("0.1.2.3", 1234);

        let params = TestAppParams {
            client_registry_capacity: 10,
            preloaded_clients: vec![(connecting_addr.clone(), "asdasd".to_string())],
            send_event: connection_request_event(connecting_addr),
        };

        make_testapp(params)
            .add_assert_system(|reg: Res<ClientRegistry>| assert_eq!(reg.client_count(), 1))
            .add_assert_system(assert_client_in_registry(connecting_addr, true))
            .run();
    }

    use crate::components::{Input, InputFlags};

    fn network_input(input: Input) -> blaminar::Bytes {
        let payload =
            westiny_common::serialization::serialize(&PacketType::InputState { input }).unwrap();
        blaminar::Bytes::from(payload)
    }

    #[test]
    fn input_commands_forwarded() {
        let mut input = Input::default();
        input.flags.set(InputFlags::RELOAD, true);
        input.flags.set(InputFlags::FORWARD, true);

        let addr = make_socket_addr("0.1.2.3", 1111);
        let params = TestAppParams {
            client_registry_capacity: 1,
            preloaded_clients: vec![(addr.clone(), "Bacsi".to_string())],
            send_event: NetworkSimulationEvent::Message(addr, network_input(input)),
        };

        make_testapp(params)
            .add_assert_system(assertion::assert_event_count::<NetworkCommand>(1))
            .add_assert_system(assertion::assert_event(NetworkCommand::Input {
                id: ClientID(0),
                input,
            }))
            .run();
    }

    #[test]
    fn not_connected_clients_input_commands_not_forwarded() {
        let mut input = Input::default();
        input.flags.set(InputFlags::RELOAD, true);
        input.flags.set(InputFlags::FORWARD, true);

        let addr = make_socket_addr("0.1.2.3", 1111);
        let params = TestAppParams {
            client_registry_capacity: 0,
            preloaded_clients: vec![],
            send_event: NetworkSimulationEvent::Message(addr, network_input(input)),
        };

        make_testapp(params)
            .add_assert_system(assertion::assert_event_count::<NetworkCommand>(0))
            .run();
    }
}
