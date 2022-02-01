use crate::components::Lifespan;
use crate::events::EntityDelete;
use bevy::prelude::{Entity, EventWriter, Query, Res, Time};

pub fn lifespan_system(
    time: Res<Time>,
    lifespans: Query<(Entity, &Lifespan)>,
    mut delete_entity: EventWriter<EntityDelete>,
) {
    for (entity, lifespan) in lifespans.iter() {
        if time.time_since_startup() >= lifespan.living_until {
            delete_entity.send(EntityDelete::new(entity));
        }
    }
}
