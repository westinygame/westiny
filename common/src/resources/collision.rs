use amethyst::ecs::Entity;
use amethyst::core::math::Vector2;
use crate::metric_dimension::length::Meter;

pub struct Collision
{
    pub collider: Entity, // moving
    pub collidee: Entity,
    pub vector: Vector2<Meter>,
}

pub struct Collisions(pub Vec<Collision>);

impl Default for Collisions {
    fn default() -> Self {
        Collisions(Vec::new())
    }
}

pub struct ProjectileCollision
{
    pub projectile: Entity,
    pub target: Entity,
    pub vector: Vector2<Meter>,
}

pub struct ProjectileCollisions(pub Vec<ProjectileCollision>);

impl Default for ProjectileCollisions {
    fn default() -> Self {
        ProjectileCollisions(Vec::new())
    }
}

