use amethyst::core::ecs::{System, ReadStorage, Entities, Write, ReadExpect, Join};
use crate::components::{Eliminated, Player, Client};
use amethyst::shrev::EventChannel;
use westiny_common::events::EntityDelete;
use crate::resources::{ClientRegistry, StreamId};
use amethyst::core::Transform;
use amethyst::shred::WriteExpect;
use amethyst::network::simulation::{TransportResource, DeliveryRequirement, UrgencyRequirement};
use westiny_common::serialize;
use westiny_common::network::{PacketType, PlayerDeath};
use amethyst::core::math::Point2;


/// Game logic related to player death
pub struct DeathSystem;

impl<'s> System<'s> for DeathSystem {
    type SystemData = (
        ReadStorage<'s, Eliminated>,
        ReadStorage<'s, Player>,
        ReadStorage<'s, Transform>,
        ReadStorage<'s, Client>,
        ReadExpect<'s, ClientRegistry>,
        Entities<'s>,
        Write<'s, EventChannel<EntityDelete>>,
        WriteExpect<'s, TransportResource>,
    );

    fn run(&mut self, data: Self::SystemData) {
        let (eliminates,
            players,
            transforms,
            clients,
            client_registry,
            entities,
            mut entity_delete_event_channel,
            mut net,
        ) = data;

        for (_eliminated, _player, transform, entity, client) in (&eliminates, &players, &transforms, &entities, &clients).join() {
            let player_name = client_registry.find_client(client.id).unwrap().player_name.clone();
            log::info!("{} died", player_name);
            // Dead player must be removed
            entity_delete_event_channel.single_write(EntityDelete {entity_id: entity});

            let death_event_msg = serialize(&PacketType::PlayerDeath(
                    PlayerDeath {
                        player_name,
                        position: Point2 {
                            coords: transform.translation().xy()
                        }
                    }
            )).expect("Could not serialize PlayerDeath");

            client_registry.get_clients().iter().for_each(|&handle| {
                net.send_with_requirements(
                    handle.addr,
                    &death_event_msg,
                    DeliveryRequirement::ReliableSequenced(StreamId::PlayerDeath.into()),
                UrgencyRequirement::OnTick);
            })
        }
    }
}
