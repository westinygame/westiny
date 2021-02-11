use amethyst::{
    derive::SystemDesc,
    ecs::{System, SystemData, Read},
    shrev::{ReaderId, EventChannel},
};
use derive_new::new;
use westiny_common::network::EntityState;
use amethyst::core::ecs::{ReadStorage, WriteStorage, Join};
use westiny_common::components::NetworkId;
use amethyst::core::Transform;
use std::collections::HashMap;

#[derive(SystemDesc, new)]
#[system_desc(name(NetworkEntityStateUpdateSystemDesc))]
pub struct NetworkEntityStateUpdateSystem {
    #[system_desc(event_channel_reader)]
    reader: ReaderId<EntityState>,
}

impl<'s> System<'s> for NetworkEntityStateUpdateSystem {
    type SystemData = (
        Read<'s, EventChannel<EntityState>>,
        ReadStorage<'s, NetworkId>,
        WriteStorage<'s, Transform>,
    );

    fn run(&mut self, (events, network_ids, mut transforms): Self::SystemData) {
        let entity_states: HashMap<_, _> = events.read(&mut self.reader).map(|entity_state| (entity_state.network_id, entity_state)).collect();

        for (net_id, transform) in (&network_ids, &mut transforms).join() {
            if let Some(state) = entity_states.get(net_id) {
                transform.set_translation_x(state.position.x);
                transform.set_translation_y(state.position.y);
                transform.set_rotation_2d(state.rotation);
            }
        }
    }
}