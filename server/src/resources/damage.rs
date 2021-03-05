use amethyst::core::ecs::Entity;
use crate::components::Damage;

pub struct DamageEvent {
    pub damage: Damage,
    pub target: Entity,
}
