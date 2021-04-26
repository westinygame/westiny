use amethyst::core::Transform;
use amethyst::core::math::{Vector2, Vector3};
use crate::components::BoundingCircle;
use crate::metric_dimension::length::{Meter, magnitude, normalize};
use std::fmt::Debug;
use crate::metric_dimension::to_meter_vec;

#[derive(Clone, Debug)]
pub struct Collider<'a>
{
    pub transform: &'a Transform,
    pub bound: &'a BoundingCircle,
}

const FUZZY_THRESHOLD : Meter = Meter(0.001 / 16.0);

pub fn check_body_collision(a: Collider, b: Collider) -> Option<Vector2<Meter>>
{
    let disposition = calculate_disposition(a.transform, b.transform);
    let distance = magnitude(disposition.clone() as Vector2<Meter>);
    let collision = a.bound.radius + b.bound.radius;
    if distance < FUZZY_THRESHOLD
    {
        Some(Vector2::new(collision, Meter(0.0)))
    }
    else if distance < collision
    {
        let colliding_line = collision - distance;
        let collision_vec = colliding_line * normalize(disposition);
        Some(collision_vec)
    }
    else
    {
        None
    }
}

pub fn check_projectile_collision(a: &Transform, b: Collider) -> Option<Vector2<Meter>>
{

    let disposition = calculate_disposition(a, b.transform);
    let distance = magnitude(disposition.clone() as Vector2<Meter>);
    let collision = b.bound.radius;
    if distance < FUZZY_THRESHOLD
    {
        Some(Vector2::new(collision, Meter(0.0)))
    }
    else if distance < collision
    {
        let colliding_line = collision - distance;
        let collision_vec = colliding_line * normalize(disposition);

        Some(collision_vec)
    }
    else
    {
        None
    }
}

fn to_vector2<T>(vec: &Vector3<T>) -> Vector2<T>
    where T: 'static + Copy + PartialEq + Debug
{
    Vector2::<T>::new( vec.x.clone(), vec.y.clone() )
}

fn calculate_disposition(a: &Transform, b: &Transform) -> Vector2<Meter>
{
    to_meter_vec(to_vector2(b.translation()) - to_vector2(a.translation()))
}

#[cfg(test)]
mod test
{
    use super::*;
    use amethyst::core::num::Zero;

    #[test]
    fn test_body_collision()
    {
        let origin = Transform::default();
        let mut point = Transform::default();
        point.set_translation_xyz(10.0, 0.0, 0.0);

        let small_bounds = BoundingCircle{radius: Meter::from_pixel(4.0)};
        let big_bounds = BoundingCircle{radius: Meter::from_pixel(6.0)};

        // no collision
        point.set_translation_xyz(10.0, 0.0, 0.0);
        assert_eq!(
            check_body_collision(
                Collider{transform: &origin, bound: &small_bounds},
                Collider{transform: &point, bound: &small_bounds}),
            None,
            "No collision");

        // regular collision
        point.set_translation_xyz(10.0, 0.0, 0.0);
        assert_eq!(
            check_body_collision(
                Collider{transform: &origin, bound: &big_bounds},
                Collider{transform: &point, bound: &big_bounds}),
            Some(Vector2::new(Meter::from_pixel(2.0), Meter::zero())),
            "Regular collision");

        // disance equals to radius
        point.set_translation_xyz(6.0, 0.0, 0.0);
        assert_eq!(
            check_body_collision(
                Collider{transform: &origin, bound: &big_bounds},
                Collider{transform: &point, bound: &big_bounds}),
            Some(Vector2::new(Meter::from_pixel(6.0), Meter::zero())),
            "Distance equals to radius");

        // matching points
        point.set_translation_xyz(0.0, 0.0, 0.0);
        assert_eq!(
            check_body_collision(
                Collider{transform: &origin, bound: &big_bounds},
                Collider{transform: &point, bound: &big_bounds}),
            Some(Vector2::new(Meter::from_pixel(12.0), Meter::zero())),
            "Matching points");

        point.set_translation_xyz(-5.0, 0.0, 0.0);
        assert_eq!(
            check_body_collision(
                Collider{transform: &origin, bound: &big_bounds},
                Collider{transform: &point, bound: &big_bounds}),
            Some(Vector2::new(Meter::from_pixel(-7.0), Meter::zero())));

        // touching outline, not considered as a collision
        point.set_translation_xyz(10.0, 0.0, 0.0);
        assert_eq!(
            check_body_collision(
                Collider{transform: &origin, bound: &big_bounds},
                Collider{transform: &point, bound: &small_bounds}),
            None,
            "Touching outline");


    }

    #[test]
    fn test_projectile_collision()
    {
        let origin = Transform::default();
        let bounds = BoundingCircle{radius: Meter::from_pixel(4.0)};

        let collider = Collider{transform: &origin, bound: &bounds};

        assert_eq!(
            check_projectile_collision(Transform::default().set_translation_xyz(0.0, 2.0, 0.0), collider.clone()),
            Some(Vector2::new(Meter::from_pixel(0.0), Meter::from_pixel(-2.0))));

        assert_eq!(
            check_projectile_collision(Transform::default().set_translation_xyz(0.0, 0.0, 0.0), collider.clone()),
            Some(Vector2::new(Meter::from_pixel(4.0), Meter::from_pixel(0.0))));

        assert_eq!(
            check_projectile_collision(Transform::default().set_translation_xyz(0.0, 4.0, 0.0), collider.clone()),
            None);

        assert_eq!(
            check_projectile_collision(Transform::default().set_translation_xyz(3.6, 3.6, 0.0), collider.clone()),
            None);
    }
}
