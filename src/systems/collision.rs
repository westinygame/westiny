use amethyst::ecs::{System, ReadStorage, WriteStorage, Entities, WriteExpect, ReadExpect};
use amethyst::core::{Transform};
use amethyst::core::math::{Vector2, Vector3};
use amethyst::ecs::prelude::Join;

use westiny_common::components::{Velocity, BoundingCircle};
use crate::resources::{Collision, Collisions, ProjectileCollision, ProjectileCollisions};
use crate::components::Projectile;

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
        for (projectile_transform, _, projectile_id) in (&transforms, &projectiles, &entities).join()
        {
            for (object_transform, object_bounds, object_id) in (&transforms, &bounding_circles, &entities).join()
            {
                // unlikely
                if projectile_id == object_id
                {
                    continue;
                }

                if let Some(collision) = check_projectile_collision(
                    projectile_transform,
                    Collider{transform: object_transform, bound: object_bounds})
                {
                    collision_resource.0.push(ProjectileCollision{
                        projectile: projectile_id,
                        target: object_id,
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
            entities.delete(collision.projectile).expect("Could not delete projectile");
        }

    }
}

#[derive(Clone)]
struct Collider<'a>
{
    transform: &'a Transform,
    bound: &'a BoundingCircle,
}

fn to_vector2(vec: &Vector3<f32>) -> Vector2<f32>
{
    Vector2::new(
        vec.x, vec.y
        )
}

fn calculate_disposition(a: &Transform, b: &Transform) -> Vector2<f32>
{
    to_vector2(b.translation()) - to_vector2(a.translation())
}

const FUZZY_THRESHOLD : f32 = 0.001;

fn check_body_collision(a: Collider, b: Collider) -> Option<Vector2<f32>>
{
    let disposition = calculate_disposition(a.transform, b.transform);
    let distance = disposition.norm();
    let collision = a.bound.radius + b.bound.radius;
    if distance < FUZZY_THRESHOLD
    {
        Some(Vector2::new(collision, 0.0))
    }
    else if distance < collision
    {
        let colliding_line = collision - distance;
        let collision_vec = (disposition / distance) * colliding_line;
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
    let distance = disposition.norm();
    let collision = b.bound.radius;
    if distance < FUZZY_THRESHOLD
    {
        Some(Vector2::new(collision, 0.0))
    }
    else if distance < collision
    {
        let colliding_line = collision - distance;
        let collision_vec = disposition / distance * colliding_line;

        Some(collision_vec)
    }
    else
    {
        None
    }
}

#[cfg(test)]
mod test
{
    use super::*;

    #[test]
    fn test_body_collision()
    {
        let origin = Transform::default();
        let mut point = Transform::default();
        point.set_translation_xyz(10.0, 0.0, 0.0);

        let small_bounds = BoundingCircle{radius: 4.0};
        let big_bounds = BoundingCircle{radius: 6.0};

        // no collision
        point.set_translation_xyz(10.0, 0.0, 0.0);
        assert_eq!(
            check_body_collision(
                Collider{transform: &origin, bound: &small_bounds},
                Collider{transform: &point, bound: &small_bounds}),
            None);

        // regular collision
        point.set_translation_xyz(10.0, 0.0, 0.0);
        assert_eq!(
            check_body_collision(
                Collider{transform: &origin, bound: &big_bounds},
                Collider{transform: &point, bound: &big_bounds}),
            Some(Vector2::new(2.0, 0.0)));

        // disance equals to radius
        point.set_translation_xyz(6.0, 0.0, 0.0);
        assert_eq!(
            check_body_collision(
                Collider{transform: &origin, bound: &big_bounds},
                Collider{transform: &point, bound: &big_bounds}),
            Some(Vector2::new(6.0, 0.0)));

        // matching points
        point.set_translation_xyz(0.0, 0.0, 0.0);
        assert_eq!(
            check_body_collision(
                Collider{transform: &origin, bound: &big_bounds},
                Collider{transform: &point, bound: &big_bounds}),
            Some(Vector2::new(12.0, 0.0)));

        point.set_translation_xyz(-5.0, 0.0, 0.0);
        assert_eq!(
            check_body_collision(
                Collider{transform: &origin, bound: &big_bounds},
                Collider{transform: &point, bound: &big_bounds}),
            Some(Vector2::new(-7.0, 0.0)));

        // touching outline, not considered as a collision
        point.set_translation_xyz(10.0, 0.0, 0.0);
        assert_eq!(
            check_body_collision(
                Collider{transform: &origin, bound: &big_bounds},
                Collider{transform: &point, bound: &small_bounds}),
            None);


    }

    #[test]
    fn test_projectile_collision()
    {
        let origin = Transform::default();
        let bounds = BoundingCircle{radius: 4.0};

        let collider = Collider{transform: &origin, bound: &bounds};

        assert_eq!(
            check_projectile_collision(Transform::default().set_translation_xyz(0.0, 2.0, 0.0), collider.clone()),
            Some(Vector2::new(0.0, -2.0)));

        assert_eq!(
            check_projectile_collision(Transform::default().set_translation_xyz(0.0, 0.0, 0.0), collider.clone()),
            Some(Vector2::new(4.0, 0.0)));

        assert_eq!(
            check_projectile_collision(Transform::default().set_translation_xyz(0.0, 4.0, 0.0), collider.clone()),
            None);

        assert_eq!(
            check_projectile_collision(Transform::default().set_translation_xyz(3.6, 3.6, 0.0), collider.clone()),
            None);
    }
}
