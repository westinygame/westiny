use amethyst::core::ecs::{System, ReadStorage, Entities, Join, Write, ReadExpect};
use crate::components::{Eliminated, Player, Client};
use amethyst::shrev::EventChannel;
use westiny_common::resources::EntityDelete;
use amethyst::core::Transform;
use crate::resources::ClientRegistry;


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
    );

    fn run(&mut self, data: Self::SystemData) {
        let (eliminates,
            players,
            transforms,
            clients,
            client_registry,
            entities,
            mut entity_delete_event_channel,
        ) = data;

        for (_eliminated, _player, _transform, entity, client) in (&eliminates, &players, &transforms, &entities, &clients).join() {
            log::info!("{} died", client_registry.find_client(client.id).unwrap().player_name);
            // Dead player must be removed
            entity_delete_event_channel.single_write(EntityDelete {entity_id: entity});
        }
    }
}