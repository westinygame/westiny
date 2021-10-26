use serde::{Serialize, Deserialize};
use derive_new::new;

#[derive(Copy, Clone, Eq, PartialEq, Debug, Serialize, Deserialize, Hash, new)]
pub struct NetworkId {
    pub entity_type: EntityType,
    pub id: u32,
}

#[derive(Copy, Clone, Debug, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub enum EntityType {
    Player,
}