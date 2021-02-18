use amethyst::{
    derive::SystemDesc,
    ecs::{System, SystemData},
    shrev::ReaderId,
    network::simulation::NetworkSimulationEvent,
};
use amethyst::core::ecs::{Read, Write};
use amethyst::core::ecs::shrev::EventChannel;
use amethyst::network::simulation::{TransportResource, DeliveryRequirement, UrgencyRequirement};
use crate::events::AppEvent;
use amethyst::core::Time;
use std::time::Duration;
use westiny_common::{network, deserialize, serialize};
use westiny_common::resources::ServerAddress;

const RUN_EVERY_N_SEC: u64 = 1;
const PLAYER_NAME_MAGIC: &str = "Narancsos_Feco";

fn get_player_name() -> String {
    std::env::var("USER").unwrap_or(PLAYER_NAME_MAGIC.to_string())
}

#[derive(SystemDesc)]
#[system_desc(name(ClientConnectSystemDesc))]
pub struct ClientConnectSystem {
    #[system_desc(event_channel_reader)]
    reader: ReaderId<NetworkSimulationEvent>,

    #[system_desc(skip)]
    last_run: Duration,
}

impl ClientConnectSystem {
    fn new(reader: ReaderId<NetworkSimulationEvent>) -> Self {
        ClientConnectSystem {
            reader,
            last_run: Duration::default()
        }
    }
}

impl<'s> System<'s> for ClientConnectSystem {
    type SystemData = (
        Read<'s, ServerAddress>,
        Read<'s, Time>,
        Write<'s, TransportResource>,
        Read<'s, EventChannel<NetworkSimulationEvent>>,
        Write<'s, EventChannel<AppEvent>>
    );

    fn run(&mut self, (server, time, mut net, net_event_ch, mut app_event): Self::SystemData) {
        let time_since_start = time.absolute_time();

        if (time_since_start-self.last_run) >= Duration::from_secs(RUN_EVERY_N_SEC) {
            self.last_run = time_since_start;
                let msg = serialize(&network::PacketType::ConnectionRequest { player_name: get_player_name() })
                    .expect("ConnectionRequest could not be serialized");

                log::debug!("Sending message. Time: {}", time_since_start.as_secs_f32());
                net.send_with_requirements(server.address, &msg, DeliveryRequirement::Reliable, UrgencyRequirement::OnTick);
        }

        for event in net_event_ch.read(&mut self.reader) {
            match event {
                NetworkSimulationEvent::Message(addr, msg) => {
                    log::debug!("Message: [{}], {:?}", addr, msg);
                    if &server.address == addr {
                        match deserialize(&msg) {
                            Ok(packet) => {
                                match packet {
                                    network::PacketType::ConnectionResponse(result) => {
                                        app_event.single_write(AppEvent::Connection(result));
                                    }
                                    _ => log::error!("Unexpected package from server: {:?}", packet)
                                }
                            }
                            Err(err) => log::error!("Connection response could not be deserialized. Cause: {:?}", err)
                        }
                    } else {
                        log::warn!("Unexpected message arrived from {} while waiting for connection response", addr);
                    }
                }
                _ => log::info!("Network event: {:?}", event)

            }
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use std::net::SocketAddr;
    use amethyst::Error;
    use amethyst::prelude::*;
    use amethyst::core::math::Point2;
    use amethyst_test::prelude::*;
    use westiny_common::components::{NetworkId, EntityType};

    const SOCKET_ADDRESS: ([u8;4], u16) = ([127, 0, 0, 1], 9999);

    #[test]
    fn writes_connected_event_on_connection_confirm() -> Result<(), Error> {
        amethyst::start_logger(Default::default());

        AmethystApplication::blank()
            .with_resource(EventChannel::<AppEvent>::new())
            .with_resource(ServerAddress { address: SocketAddr::from(SOCKET_ADDRESS) })
            .with_setup(move |world: &mut World| {
                let reader_id = world.fetch_mut::<EventChannel<AppEvent>>().register_reader();
                world.insert(reader_id);
            })
            .with_effect(|world| {
                let mut network_event_channel = world.fetch_mut::<EventChannel<NetworkSimulationEvent>>();
                network_event_channel.single_write(
                    NetworkSimulationEvent::Message(
                        SocketAddr::from(SOCKET_ADDRESS),
                        serialize(&connection_response()).unwrap().into()
                    )
                );
            })
            .with_system_desc(ClientConnectSystemDesc::default(), "client_connect_sys", &[])

            .with_assertion(move |world: &mut World| {
                let app_event_channel = world.fetch_mut::<EventChannel<AppEvent>>();
                let mut reader_id = world.write_resource::<ReaderId<AppEvent>>();

                let events = app_event_channel.read(&mut reader_id);
                assert_eq!(events.len(), 1, "There should be exactly 1 AppEvent written");
                let expected_response: network::Result<network::ClientInitialData> = Ok(
                    network::ClientInitialData{
                        player_network_id: NetworkId::new(EntityType::Player, 0),
                        initial_pos: Point2::from([0.0, 0.0]),
                    });
                assert_eq!(events.collect::<Vec<&AppEvent>>()[0], &AppEvent::Connection(expected_response))
            })
            .run()
    }

    #[inline]
    fn connection_response() -> network::PacketType {
        network::PacketType::ConnectionResponse(
            Ok(
                network::ClientInitialData {
                    player_network_id: NetworkId::new(EntityType::Player, 0),
                    initial_pos: Point2::from([0.0, 0.0])
                }
            )
        )
    }
}
