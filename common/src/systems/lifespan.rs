use amethyst::shred::{System, ReadExpect};
use amethyst::core::Time;
use amethyst::core::ecs::{ReadStorage, Join, Entities, Write};
use crate::components::Lifespan;
use crate::events::EntityDelete;
use amethyst::shrev::EventChannel;

pub struct LifespanSystem;

impl<'s> System<'s> for LifespanSystem {
    type SystemData = (
        ReadExpect<'s, Time>,
        ReadStorage<'s, Lifespan>,
        Write<'s, EventChannel<EntityDelete>>,
        Entities<'s>,
    );

    fn run(&mut self, data: Self::SystemData) {
        let (
            time,
            time_limits,
            mut delete_event_channel,
            entities,
        ) = data;

        let abs_time = time.absolute_time();
        for (limit, entity) in (&time_limits, &entities).join() {
            if abs_time >= limit.living_until {
                log::info!("Delete entity. Lifespan");
                delete_event_channel.single_write(EntityDelete{ entity_id: entity });
            }
        }
    }
}