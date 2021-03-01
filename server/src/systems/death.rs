use amethyst::core::ecs::{System, ReadStorage, Entities, Join, Write};
use crate::components::{Eliminated, Player};
use amethyst::shrev::EventChannel;
use westiny_common::resources::EntityDelete;
use amethyst::core::Transform;



/// Game logic related to player death
pub struct DeathSystem;

impl<'s> System<'s> for DeathSystem {
    type SystemData = (
        ReadStorage<'s, Eliminated>,
        ReadStorage<'s, Player>,
        ReadStorage<'s, Transform>,
        Entities<'s>,
        Write<'s, EventChannel<EntityDelete>>,
    );

    fn run(&mut self, data: Self::SystemData) {
        let (eliminates,
            players,
            transforms,
            entities,
            mut entity_delete_event_channel,
        ) = data;

        for (_eliminated, _player, _transform, entity) in (&eliminates, &players, &transforms, &entities).join() {
            // Dead player must be removed
            entity_delete_event_channel.single_write(EntityDelete {entity_id: entity});
        }
    }
}