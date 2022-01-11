use bevy::ecs::prelude::Entity;
use crate::components::Damage;

pub struct DamageEvent {
    pub damage: Damage,
    pub target: Entity,
}
