use amethyst::core::ecs::{System, ReadStorage, Entities, Join, Write, ReadExpect, LazyUpdate, WriteExpect};
use crate::components::{Eliminated, Player, Client, EntityType};
use amethyst::shrev::EventChannel;
use westiny_common::resources::EntityDelete;
use amethyst::core::Transform;
use crate::resources::{ClientRegistry, NetworkIdSupplier};
use amethyst::prelude::Builder;


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
        ReadExpect<'s, LazyUpdate>,
        WriteExpect<'s, NetworkIdSupplier>,
    );

    fn run(&mut self, data: Self::SystemData) {
        let (eliminates,
            players,
            transforms,
            clients,
            client_registry,
            entities,
            mut entity_delete_event_channel,
            lazy,
            mut network_id_supplier,
        ) = data;

        for (_eliminated, _player, transform, entity, client) in (&eliminates, &players, &transforms, &entities, &clients).join() {
            log::info!("{} died", client_registry.find_client(client.id).unwrap().player_name);
            // Dead player must be removed
            entity_delete_event_channel.single_write(EntityDelete {entity_id: entity});

            lazy.create_entity(&entities)
                .with(transform.clone())
                .with(network_id_supplier.next(EntityType::Corpse))
                .build();
        }
    }
}