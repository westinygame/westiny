use bevy::prelude::{Res, ResMut, State, EventReader};
use blaminar::simulation::{TransportResource, NetworkSimulationEvent, DeliveryRequirement, UrgencyRequirement};
use crate::states::AppState;
use crate::resources::ServerAddress;
use westiny_common::serialization::{deserialize, serialize};
use westiny_common::network::PacketType;

const PLAYER_NAME_MAGIC: &str = "Narancsos_Feco";

fn get_player_name() -> String {
    std::env::var("USER").unwrap_or(PLAYER_NAME_MAGIC.to_string())
}

pub fn connect_to_server(
    server_addr: Res<ServerAddress>,
    mut net: ResMut<TransportResource>,
    mut net_event: EventReader<NetworkSimulationEvent>,
    mut app_state: ResMut<State<AppState>>)
{
    for event in net_event.iter() {
        match event {
            NetworkSimulationEvent::Message(addr, msg) => {
                log::debug!("Message: [{}], {:?}", addr, msg);
                if server_addr.address == *addr {
                    match deserialize(&msg) {
                        Ok(packet) => {
                            match packet {
                                PacketType::ConnectionResponse(Ok(init_data)) => app_state.set(AppState::InGame(init_data)).unwrap(),
                                PacketType::ConnectionResponse(Err(err)) => log::error!("Conection refused. Reason: {}", err),
                                _ => log::error!("Unexpected package from server: {:02x?}", packet)
                            }
                        }
                        Err(err) => log::error!("Connection response could not be deserialized. Cause: {:?}", err)
                    }
                } else {
                    log::warn!("Unexpected message arrived from unknown sender {} while waiting for connection response from server: {}", addr, server_addr.address);
                }
            }
            _ => log::info!("Network event: {:?}", event)

        }
        return;
    }

    log::info!("Trying to connect to server: {:?}", server_addr.address);
    let msg = serialize(&PacketType::ConnectionRequest { player_name: get_player_name() })
        .expect("ConnectionRequest could not be serialized");
    net.send_with_requirements(server_addr.address, &msg, DeliveryRequirement::ReliableSequenced(None), UrgencyRequirement::OnTick);
}

#[cfg(test)]
mod test {
    use super::*;
    use std::net::SocketAddr;
    use westiny_common::components::{NetworkId, EntityType};
    use westiny_common::resources::Seed;
    use westiny_common::network::{self, PacketType};
    use westiny_test::*;
    use bevy::prelude::App;

    const SOCKET_ADDRESS: ([u8;4], u16) = ([127, 0, 0, 1], 9999);

    #[test]
    fn writes_connected_event_on_connection_confirm() {
        App::new()
            .add_state(AppState::Connect)
            .init_resource::<TransportResource>()
            .insert_resource(ServerAddress { address: SocketAddr::from(SOCKET_ADDRESS)})
            .send_events(
                vec![
                    Some(NetworkSimulationEvent::Message(
                            SocketAddr::from(SOCKET_ADDRESS),
                            serialize(
                                &PacketType::ConnectionResponse(ok_init_data())
                            ).unwrap().into()
                        )
                    )
                ]
            )
            .add_assert_system(
                assertion::assert_current_state(
                AppState::InGame(
                    network::ClientInitialData {
                        player_network_id: NetworkId::new(EntityType::Player, 0),
                        seed: Seed(100),
                    })))
            .add_system(connect_to_server)
            .run();
    }

    #[inline]
    fn ok_init_data() -> network::Result<network::ClientInitialData> {
            Ok(
                network::ClientInitialData {
                    player_network_id: NetworkId::new(EntityType::Player, 0),
                    seed: Seed(100),
                }
            )
    }

    #[test]
    fn sends_connection_request() {
        App::new()
            .add_state(AppState::Connect)
            .init_resource::<TransportResource>()
            .insert_resource(ServerAddress { address: SocketAddr::from(SOCKET_ADDRESS)})
            .send_events(
                vec![
                    Some(NetworkSimulationEvent::Message(
                            SocketAddr::from(SOCKET_ADDRESS),
                            serialize(
                                &PacketType::ConnectionResponse(ok_init_data())
                            ).unwrap().into()
                        )
                    )
                ]
            )
            .add_system(connect_to_server)
            .run();
    }
}
