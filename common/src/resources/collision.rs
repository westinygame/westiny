use crate::metric_dimension::length::MeterVec2;
use bevy::prelude::{Entity, Resource};

pub struct Collision {
    pub collider: Entity, // moving
    pub collidee: Entity,
    pub vector: MeterVec2,
}

#[derive(Default, Resource)]
pub struct Collisions(pub Vec<Collision>);

pub struct ProjectileCollision {
    pub projectile: Entity,
    pub target: Entity,
    pub vector: MeterVec2,
}

#[derive(Default, Resource)]
pub struct ProjectileCollisions(pub Vec<ProjectileCollision>);
