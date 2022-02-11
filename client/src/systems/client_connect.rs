use crate::resources::ServerAddress;
use crate::states::AppState;
use bevy::prelude::{EventReader, Res, ResMut, State, Local, Time};
use blaminar::simulation::{
    DeliveryRequirement,
    NetworkSimulationEvent,
    TransportResource,
    UrgencyRequirement,
};
use westiny_common::network::PacketType::{ConnectionRequest, ConnectionResponse, EntityStateUpdate};
use westiny_common::serialization::{deserialize, serialize};

const PLAYER_NAME_MAGIC: &str = "Narancsos_Feco";

fn get_player_name() -> String {
    std::env::var("USER").unwrap_or(PLAYER_NAME_MAGIC.to_string())
}

#[derive(Default)]
pub struct LastRun(std::time::Duration);

pub fn send_connection_request(
    server_addr: Res<ServerAddress>,
    mut net: ResMut<TransportResource>,
    time: Res<Time>,
    mut last_run: Local<LastRun>)
{
    if time.time_since_startup() - last_run.0 < std::time::Duration::from_secs(1u64) {
        return;
    }
    last_run.0 = time.time_since_startup();

    log::info!("Trying to connect to server: {:?}", server_addr.address);
    let msg = serialize(&ConnectionRequest {
        player_name: get_player_name(),
    })
    .expect("ConnectionRequest could not be serialized");
    net.send_with_requirements(
        server_addr.address,
        &msg,
        DeliveryRequirement::ReliableSequenced(None),
        UrgencyRequirement::OnTick,
    );
}

pub fn receive_connection_response(
    server_addr: Res<ServerAddress>,
    mut net_event: EventReader<NetworkSimulationEvent>,
    mut app_state: ResMut<State<AppState>>)
{
    for event in net_event.iter() {
        match event {
            NetworkSimulationEvent::Message(addr, msg) => {
                log::debug!("Message: [{}], {:?}", addr, msg);
                if server_addr.address != *addr {
                    log::warn!("Unexpected message arrived from unknown sender {} while waiting for connection response from server: {}", addr, server_addr.address);
                    continue;
                }

                match deserialize(&msg) {
                    Ok(packet) => match packet {
                        ConnectionResponse(Ok(init_data)) => {
                            log::info!("Connection established");
                            app_state.set(AppState::Play)
                                .expect("Failed to set AppState to Play");
                            return;
                        }
                        ConnectionResponse(Err(err)) => {
                            log::error!("Conection refused. Reason: {}", err)
                        }
                        //EntityStateUpdate(_) => log::debug!("EntityStateUpdate received"),
                        _ => log::error!("Unexpected package from server: {:02x?}", packet),
                    },
                    Err(err) => log::error!(
                        "Connection response could not be deserialized. Cause: {:?}",
                        err
                    ),
                }
            }
            _ => log::info!("Network event: {:?}", event),
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use bevy::prelude::App;
    use std::net::SocketAddr;
    use w_bevy_test::*;
    use westiny_common::components::{EntityType, NetworkId};
    use westiny_common::network::{self, PacketType};
    use westiny_common::resources::Seed;

    const SOCKET_ADDRESS: ([u8; 4], u16) = ([127, 0, 0, 1], 9999);

    #[test]
    fn writes_connected_event_on_connection_confirm() {
        App::new()
            .add_state(AppState::Connect)
            .init_resource::<TransportResource>()
            .insert_resource(ServerAddress {
                address: SocketAddr::from(SOCKET_ADDRESS),
            })
            .send_event(NetworkSimulationEvent::Message(
                SocketAddr::from(SOCKET_ADDRESS),
                serialize(&PacketType::ConnectionResponse(ok_init_data()))
                    .unwrap()
                    .into(),
            ))
            .add_assert_system(assertion::assert_current_state(AppState::InGame(
                network::ClientInitialData {
                    player_network_id: NetworkId::new(EntityType::Player, 0),
                    seed: Seed(100),
                },
            )))
            .add_system(connect_to_server)
            .run();
    }

    #[inline]
    fn ok_init_data() -> network::Result<network::ClientInitialData> {
        Ok(network::ClientInitialData {
            player_network_id: NetworkId::new(EntityType::Player, 0),
            seed: Seed(100),
        })
    }

    #[test]
    fn sends_connection_request() {
        App::new()
            .add_state(AppState::Connect)
            .init_resource::<TransportResource>()
            .insert_resource(ServerAddress {
                address: SocketAddr::from(SOCKET_ADDRESS),
            })
            .send_events(vec![Some(NetworkSimulationEvent::Message(
                SocketAddr::from(SOCKET_ADDRESS),
                serialize(&PacketType::ConnectionResponse(ok_init_data()))
                    .unwrap()
                    .into(),
            ))])
            .add_system(connect_to_server)
            .run();
    }
}
