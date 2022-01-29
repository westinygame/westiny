use bevy::ecs::prelude::Entity;
use derive_new::new;

#[derive(new)]
pub struct EntityDelete
{
    pub entity_id: Entity,
}
