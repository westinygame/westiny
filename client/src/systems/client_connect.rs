use crate::resources::{PlayerNetworkId, Seed, ServerAddress};
use crate::states::AppState;
use bevy::prelude::{EventReader, Local, Res, ResMut, State, Time};
use blaminar::simulation::{
    DeliveryRequirement, NetworkSimulationEvent, TransportResource, UrgencyRequirement,
};
use westiny_common::network::PacketType::{ConnectionRequest, ConnectionResponse};
use westiny_common::serialization::{deserialize, serialize};
use std::time::Duration;

const PLAYER_NAME_MAGIC: &str = "Narancsos_Feco";

fn get_player_name() -> String {
    std::env::var("USER").unwrap_or_else(|_| PLAYER_NAME_MAGIC.to_string())
}

#[derive(Default)]
pub struct LastRun(Duration);

pub fn send_connection_request(
    server_addr: Res<ServerAddress>,
    mut net: ResMut<TransportResource>,
    time: Res<Time>,
    mut last_run: Local<LastRun>,
) {
    // First condition is to avoid 1sec dead time after first system run
    if last_run.0 != Duration::ZERO && time.time_since_startup() - last_run.0 < Duration::from_secs(1u64) {
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
    mut app_state: ResMut<State<AppState>>,
    mut seed: ResMut<Seed>,
    mut player_network_id: ResMut<PlayerNetworkId>,
) {
    for event in net_event.iter() {
        match event {
            NetworkSimulationEvent::Message(addr, msg) => {
                log::debug!("Message: [{}], {:?}", addr, msg);
                if server_addr.address != *addr {
                    log::warn!("Unexpected message arrived from unknown sender {} while waiting for connection response from server: {}", addr, server_addr.address);
                    continue;
                }

                match deserialize(msg) {
                    Ok(packet) => match packet {
                        ConnectionResponse(Ok(init_data)) => {
                            log::info!("Connection established");
                            app_state
                                .set(AppState::PlayInit)
                                .expect("Failed to set AppState to PlayInit");
                            *seed = init_data.seed;
                            player_network_id.0 = init_data.player_network_id;
                            return;
                        }
                        ConnectionResponse(Err(err)) => {
                            log::error!("Conection refused. Reason: {}", err)
                        }
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
    fn sends_connection_request() {
        std::env::set_var("USER", "abcd1234");

        let expected_payload = serialize(&ConnectionRequest {
            player_name: "abcd1234".to_string(),
        }).unwrap();

        println!("Start test");
        App::new()
            .add_plugin(bevy::core::CorePlugin)
            .init_resource::<TransportResource>()
            .insert_resource(ServerAddress {
                address: SocketAddr::from(SOCKET_ADDRESS),
            })
            .add_assert_system(move |net: Res<TransportResource>| {
                                    assert_eq!(net.get_messages().len(), 1);
                                    assert_eq!(
                                        net.get_messages()[0],
                                        blaminar::simulation::Message {
                                            destination: SocketAddr::from(SOCKET_ADDRESS),
                                            payload: blaminar::Bytes::copy_from_slice(&expected_payload),
                                            delivery: DeliveryRequirement::ReliableSequenced(None),
                                            urgency: UrgencyRequirement::OnTick,
                                        }
                                    )
                               })
            .add_system(send_connection_request)
            .run();
    }

    #[inline]
    fn ok_init_data() -> network::Result<network::ClientInitialData> {
        Ok(network::ClientInitialData {
            player_network_id: NetworkId::new(EntityType::Player, 1234),
            seed: Seed(100),
        })
    }

    #[test]
    fn sets_app_state_and_resources_on_connection_confirm() {
        App::new()
            .add_state(AppState::Connect)
            .init_resource::<TransportResource>()
            .init_resource::<Seed>()
            .init_resource::<PlayerNetworkId>()
            .insert_resource(ServerAddress {
                address: SocketAddr::from(SOCKET_ADDRESS),
            })
            .send_event(NetworkSimulationEvent::Message(
                SocketAddr::from(SOCKET_ADDRESS),
                serialize(&PacketType::ConnectionResponse(ok_init_data()))
                    .unwrap()
                    .into(),
            ))
            .add_assert_system(assertion::assert_current_state(AppState::Play))
            .add_assert_system(assertion::assert_resource(Seed(100)))
            .add_assert_system(assertion::assert_resource(PlayerNetworkId(NetworkId::new(EntityType::Player, 1234))))

            .send_events(vec![Some(NetworkSimulationEvent::Message(
                SocketAddr::from(SOCKET_ADDRESS),
                serialize(&PacketType::ConnectionResponse(ok_init_data()))
                    .unwrap()
                    .into(),
            ))])
            .add_system(receive_connection_response)
            .run();
    }
}
