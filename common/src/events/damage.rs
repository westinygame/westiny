use crate::components::Damage;
use bevy::ecs::prelude::Entity;

pub struct DamageEvent {
    pub damage: Damage,
    pub target: Entity,
}
