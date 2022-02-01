use bevy::ecs::component::Component;
use derive_new::new;
use serde::{Deserialize, Serialize};

#[derive(Copy, Clone, Eq, PartialEq, Debug, Serialize, Deserialize, Hash, new, Component)]
pub struct NetworkId {
    pub entity_type: EntityType,
    pub id: u32,
}

#[derive(Copy, Clone, Debug, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub enum EntityType {
    Player,
}
