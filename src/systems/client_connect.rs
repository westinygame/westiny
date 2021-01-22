use amethyst::{
    derive::SystemDesc,
    ecs::{System, SystemData},
    shrev::ReaderId,
    network::simulation::NetworkSimulationEvent,
};
use amethyst::core::ecs::{Read, Write};
use amethyst::core::ecs::shrev::EventChannel;
use amethyst::network::simulation::TransportResource;
use crate::events::AppEvent;
use amethyst::core::Time;
use std::net::{IpAddr, Ipv4Addr, SocketAddr};
use std::str::FromStr;
use std::time::Duration;
use crate::states::client_states::ServerAddress;

const RUN_EVERY_N_SEC: u64 = 1;

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

    fn run(&mut self, (server, time, mut net, network_event, mut app_event): Self::SystemData) {
        let time_since_start = time.absolute_time();

        if (time_since_start-self.last_run) >= Duration::from_secs(RUN_EVERY_N_SEC) {
            self.last_run = time_since_start;
            if let Some(addr) = server.address {
                let msg = [1u8, 2, 3, 4];
                log::info!("Sending message. Time: {}", time_since_start.as_secs_f32());
                net.send(addr, &msg);
            }
        }
    }
}
