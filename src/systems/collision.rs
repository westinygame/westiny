use amethyst::ecs::{System, ReadStorage, WriteStorage, Entities, WriteExpect, ReadExpect};
use amethyst::core::{Transform};
use amethyst::core::math::{Vector2};
use amethyst::ecs::prelude::Join;

use crate::components::{Velocity, BoundingCircle, Projectile};
use crate::resources::{Collision, Collisions, ProjectileCollision, ProjectileCollisions};

use log::info;

pub struct CollisionSystem;

impl<'s> System<'s> for CollisionSystem {
    type SystemData = (
        WriteStorage<'s, Transform>,
        ReadStorage<'s, Velocity>,
        ReadStorage<'s, BoundingCircle>,
        Entities<'s>,
        WriteExpect<'s, Collisions>
        );

    fn run(&mut self, (transforms, velocities, bounding_circles, entities, mut collision_resource): Self::SystemData) {
        collision_resource.0.clear();
        for (moving_id, moving_transform, moving_bounds, _) in (&entities, &transforms, &bounding_circles, &velocities).join()
        {
            // NOTE: this is not necessarily standing
            for (standing_id, standing_transform, standing_bounds) in (&entities, &transforms, &bounding_circles).join()
            {
                // Do not collide with itself
                if moving_id == standing_id
                {
                    continue;
                }

                if let Some(collision) = check_body_collision(
                    Collider{transform: moving_transform, bound: moving_bounds},
                    Collider{transform: standing_transform, bound: standing_bounds})
                {
                    info!("Collision {}", collision);

                    collision_resource.0.push(Collision{collider: moving_id, collidee: standing_id, vector: collision});
                }
            }
        }
    }
}

pub struct CollisionHandlerForObstacles;

impl<'s> System<'s> for CollisionHandlerForObstacles {
    type SystemData = (
        ReadExpect<'s, Collisions>,
        WriteStorage<'s, Transform>
        );

    fn run(&mut self, (collisions, mut transforms): Self::SystemData) {
        for collision in &collisions.0 {
            if let Some(transform) = transforms.get_mut(collision.collider)
            {
                transform.prepend_translation_x(-collision.vector.x);
                transform.prepend_translation_y(-collision.vector.y);
            }
        }

    }
}

pub struct ProjectileCollisionSystem;

impl<'s> System<'s> for ProjectileCollisionSystem {
    type SystemData = (
        WriteStorage<'s, Transform>,
        ReadStorage<'s, Projectile>,
        ReadStorage<'s, BoundingCircle>,
        Entities<'s>,
        WriteExpect<'s, ProjectileCollisions>
        );
    fn run(&mut self, (transforms, projectiles, bounding_circles, entities, mut collision_resource): Self::SystemData) {
        collision_resource.0.clear();
        for projectile in (&transforms, &projectiles, &entities).join()
        {
            for object in (&transforms, &bounding_circles, &entities).join()
            {
                // unlikely
                if projectile.2 == object.2
                {
                    continue;
                }

                if let Some(collision) = check_projectile_collision(
                    projectile.0,
                    Collider{transform: object.0, bound: object.1})
                {
                    collision_resource.0.push(ProjectileCollision{
                        projectile: projectile.2,
                        target: object.2,
                        vector: collision});
                }
            }
        }
    }
}

pub struct ProjectileCollisionHandler;

impl<'s> System<'s> for ProjectileCollisionHandler {
    type SystemData = (
        Entities<'s>,
        ReadExpect<'s, ProjectileCollisions>
        );

    // Here Projectile components are not explicitly filtered. ProjectCollisionSystem is expected
    // to put proper entities in `collision.projectile`
    fn run(&mut self, (entities, collisions): Self::SystemData) {
        for collision in &collisions.0 {
            entities.delete(collision.projectile).expect("JAJAJa");
        }

    }
}

struct Collider<'a>
{
    transform: &'a Transform,
    bound: &'a BoundingCircle,
}

fn calculate_disposition(a: &Transform, b: &Transform) -> Vector2<f32>
{
    Vector2::new(
        b.translation().x - a.translation().x,
        b.translation().y - a.translation().y
        )
}

fn calculate_length(vec: &Vector2<f32>) -> f32
{
    let mut distance = (vec.x.powf(2.0) + vec.y.powf(2.0)).sqrt();
    if distance == 0.0
    {
        distance = 0.0001;
    }
    distance
}

fn check_body_collision(a: Collider, b: Collider) -> Option<Vector2<f32>>
{
    let disposition = calculate_disposition(a.transform, b.transform);
    let distance = calculate_length(&disposition);
    let collision = a.bound.radius + b.bound.radius;
    if distance < collision
    {
        let colliding_line = collision - distance;
        let collision_vec = disposition.normalize() * colliding_line;

        Some(collision_vec)
    }
    else
    {
        None
    }
}

fn check_projectile_collision(a: &Transform, b: Collider) -> Option<Vector2<f32>>
{
    let disposition = calculate_disposition(a, b.transform);
    let distance = calculate_length(&disposition);
    let collision = b.bound.radius;
    if distance < collision
    {
        let colliding_line = collision - distance;
        let collision_vec = disposition.normalize() * colliding_line;

        Some(collision_vec)
    }
    else
    {
        None
    }
}
