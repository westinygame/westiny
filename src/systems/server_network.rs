use amethyst::{
    derive::SystemDesc,
    ecs::{System, SystemData},
    shrev::ReaderId,
    network::simulation::NetworkSimulationEvent,
};
use amethyst::core::ecs::{Write, Read};
use amethyst::core::ecs::shrev::EventChannel;
use amethyst::network::simulation::{TransportResource, DeliveryRequirement, UrgencyRequirement};
use std::net::SocketAddr;
use crate::network;
use amethyst::core::math::Point2;

#[derive(SystemDesc)]
#[system_desc(name(ServerNetworkSystemDesc))]
pub struct ServerNetworkSystem {
    #[system_desc(event_channel_reader)]
    reader: ReaderId<NetworkSimulationEvent>,
}

impl ServerNetworkSystem {
    pub fn new(reader: ReaderId<NetworkSimulationEvent>) -> Self {
        Self { reader }
    }
}

impl<'s> System<'s> for ServerNetworkSystem {
    type SystemData = (
        Write<'s, TransportResource>,
        Read<'s, EventChannel<NetworkSimulationEvent>>,
    );

    fn run(&mut self, (mut net, net_event_ch): Self::SystemData) {
        for event in net_event_ch.read(&mut self.reader) {
            match event {
                NetworkSimulationEvent::Message(addr, payload) => {
                    log::info!("Message: {:?}", payload);
                    match bincode::deserialize(payload) {
                        Ok(msg) => {
                            if let Some(response_payload) = self.handle_message(addr, &msg) {
                                net.send_with_requirements(*addr, &response_payload, DeliveryRequirement::Reliable, UrgencyRequirement::OnTick);
                            }
                        },
                        Err(err) => {
                            log::error!("Message from {} could not be deserialized. Cause: {}", addr, err);
                        }
                    }
                }
                _ => log::info!("Network event: {:?}", event)

            }
        }
    }


}

impl ServerNetworkSystem {
    // TODO return result
    // TODO should it determine the delivery & urgency requirements?
    fn handle_message(&self, address: &SocketAddr, message: &network::PackageType) -> Option<Vec<u8>> {
        use network::*;

        match message {
            PackageType::ConnectionRequest{ player_name } => {
                log::debug!("Connection request received: {}, {}", address, player_name);
                // TODO place checks here (blacklisted, already connected, different version, etc.)
                // TODO store somewhere as connected client
                // TODO load last position or generate brand new
                let response = PackageType::ConnectionResponse(Ok(ClientInitialData::new(Point2::new(0.0, 0.0))));
                Some(bincode::serialize(&response).expect("Response to connection request could not be serialized"))
            }
            unexpected_msg => {
                log::error!("Unexpected message from {}", address);
                log::debug!("Unexpected {}: {:?}", address, unexpected_msg);
                None
            }
        }
    }
}