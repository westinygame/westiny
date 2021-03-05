use derive_new::new;
use amethyst::ecs::{Read, System, SystemData, Entities};
use amethyst::derive::SystemDesc;
use amethyst::shrev::{ReaderId, EventChannel};

use crate::resources::EntityDelete;

#[derive(SystemDesc, new)]
#[system_desc(name(EntityDeleteSystemDesc))]
pub struct EntityDeleteSystem {
    #[system_desc(event_channel_reader)]
    reader: ReaderId<EntityDelete>,
}

impl<'s> System<'s> for EntityDeleteSystem {
    type SystemData = (
        Read<'s, EventChannel<EntityDelete>>,
        Entities<'s>);

    fn run(&mut self, (id_channel, entities): Self::SystemData) {
        for EntityDelete{entity_id} in id_channel.read(&mut self.reader) {
            entities.delete(*entity_id).expect("Could not delete entity!");
        }
    }
}
