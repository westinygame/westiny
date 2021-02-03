use amethyst::ecs::Entity;
use amethyst::core::math::Vector2;

pub struct Collision
{
    pub collider: Entity, // moving
    pub collidee: Entity,
    pub vector: Vector2<f32>,
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
    pub vector: Vector2<f32>,
}

pub struct ProjectileCollisions(pub Vec<ProjectileCollision>);

impl Default for ProjectileCollisions {
    fn default() -> Self {
        ProjectileCollisions(Vec::new())
    }
}

