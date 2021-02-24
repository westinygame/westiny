

use amethyst::core::Transform;
use amethyst::core::math::{Vector2, Vector3};
use crate::components::BoundingCircle;

#[derive(Clone)]
pub struct Collider<'a>
{
    pub transform: &'a Transform,
    pub bound: &'a BoundingCircle,
}


const FUZZY_THRESHOLD : f32 = 0.001;

pub fn check_body_collision(a: Collider, b: Collider) -> Option<Vector2<f32>>
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

pub fn check_projectile_collision(a: &Transform, b: Collider) -> Option<Vector2<f32>>
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
