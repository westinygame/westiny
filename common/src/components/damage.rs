use bevy::ecs::component::Component;

#[derive(Copy, Clone, Debug, Component)]
pub struct Damage(pub u16);
