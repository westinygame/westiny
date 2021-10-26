use crate::metric_dimension::length::MeterVec2;
use bevy::prelude::Entity;

pub struct Collision
{
    pub collider: Entity, // moving
    pub collidee: Entity,
    pub vector: MeterVec2,
}

pub struct Collisions(pub Vec<Collision>);

impl Default for Collisions {
    fn default() -> Self {
        Collisions(Vec::new())
    }
}

pub struct ProjectileCollision {
    pub projectile: Entity,
    pub target: Entity,
    pub vector: MeterVec2,
}

pub struct ProjectileCollisions(pub Vec<ProjectileCollision>);

impl Default for ProjectileCollisions {
    fn default() -> Self {
        ProjectileCollisions(Vec::new())
    }
}

