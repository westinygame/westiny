pub use health::HealthUpdateSystemDesc;
pub use physics::PhysicsSystem;
pub use lifespan::LifespanSystem;
pub use collision::{
    CollisionBundle,
    CollisionHandlerForObstacles,
    CollisionSystem,
    ProjectileCollisionHandler,
    ProjectileCollisionSystem
};

mod health;
mod physics;
mod lifespan;
mod collision;