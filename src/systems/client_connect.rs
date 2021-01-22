use amethyst::{
    derive::SystemDesc,
    ecs::{System, SystemData},
    shrev::ReaderId,
    network::simulation::NetworkSimulationEvent,
};

#[derive(SystemDesc)]
#[system_desc(name(ClientConnectSystemDesc))]
pub struct ClientConnectSystem {
    #[system_desc(event_channel_reader)]
    reader: ReaderId<NetworkSimulationEvent>,
}

impl ClientConnectSystem {
    fn new(reader: ReaderId<NetworkSimulationEvent>) -> Self {
        ClientConnectSystem { reader }
    }
}

impl<'s> System<'s> for ClientConnectSystem {
    type SystemData = ();

    fn run(&mut self, _data: Self::SystemData) {
        log::info!("Connect system");
    }
}
