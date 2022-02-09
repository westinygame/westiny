use crate::components::BoundingCircle;
use crate::metric_dimension::length::{magnitude, normalize, Meter, MeterVec2};
use bevy::prelude::*;
use std::fmt::Debug;

#[derive(Clone, Debug)]
pub struct Collider<'a> {
    pub transform: &'a Transform,
    pub bound: &'a BoundingCircle,
}

const FUZZY_THRESHOLD: Meter = Meter(0.001 / 16.0);

pub fn check_body_collision(a: Collider, b: Collider) -> Option<MeterVec2> {
    let disposition = calculate_disposition(a.transform, b.transform);
    let distance = magnitude(&disposition);
    let collision = a.bound.radius + b.bound.radius;
    if distance < FUZZY_THRESHOLD {
        Some(MeterVec2 {
            x: collision,
            y: Meter(0.0),
        })
    } else if distance < collision {
        let colliding_line = collision - distance;
        let collision_vec = colliding_line * normalize(disposition);
        Some(collision_vec)
    } else {
        None
    }
}

pub fn check_projectile_collision(a: &Transform, b: Collider) -> Option<MeterVec2> {
    let disposition = calculate_disposition(a, b.transform);
    let distance = magnitude(&disposition);
    let collision = b.bound.radius;
    if distance < FUZZY_THRESHOLD {
        Some(MeterVec2 {
            x: collision,
            y: Meter(0.0),
        })
    } else if distance < collision {
        let colliding_line = collision - distance;
        let collision_vec = colliding_line * normalize(disposition);

        Some(collision_vec)
    } else {
        None
    }
}

fn calculate_disposition(a: &Transform, b: &Transform) -> MeterVec2 {
    MeterVec2::from_pixel_vec(b.translation.truncate() - a.translation.truncate())
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_body_collision() {
        let origin = Transform::default();
        let mut point = Transform::from_xyz(10.0, 0.0, 0.0);

        let small_bounds = BoundingCircle {
            radius: Meter::from_pixel(4.0),
        };
        let big_bounds = BoundingCircle {
            radius: Meter::from_pixel(6.0),
        };

        // no collision
        point = Transform::from_xyz(10.0, 0.0, 0.0);
        assert_eq!(
            check_body_collision(
                Collider {
                    transform: &origin,
                    bound: &small_bounds
                },
                Collider {
                    transform: &point,
                    bound: &small_bounds
                }
            ),
            None,
            "No collision"
        );

        // regular collision
        point = Transform::from_xyz(10.0, 0.0, 0.0);
        assert_eq!(
            check_body_collision(
                Collider {
                    transform: &origin,
                    bound: &big_bounds
                },
                Collider {
                    transform: &point,
                    bound: &big_bounds
                }
            ),
            Some(MeterVec2::from_pixel_vec(Vec2::new(2.0, 0.0))),
            "Regular collision"
        );

        // disance equals to radius
        point = Transform::from_xyz(6.0, 0.0, 0.0);
        assert_eq!(
            check_body_collision(
                Collider {
                    transform: &origin,
                    bound: &big_bounds
                },
                Collider {
                    transform: &point,
                    bound: &big_bounds
                }
            ),
            Some(MeterVec2::from_pixel_vec(Vec2::new(6.0, 0.0))),
            "Distance equals to radius"
        );

        // matching points
        point = Transform::from_xyz(0.0, 0.0, 0.0);
        assert_eq!(
            check_body_collision(
                Collider {
                    transform: &origin,
                    bound: &big_bounds
                },
                Collider {
                    transform: &point,
                    bound: &big_bounds
                }
            ),
            Some(MeterVec2::from_pixel_vec(Vec2::new(12.0, 0.0))),
            "Matching points"
        );

        point = Transform::from_xyz(-5.0, 0.0, 0.0);
        assert_eq!(
            check_body_collision(
                Collider {
                    transform: &origin,
                    bound: &big_bounds
                },
                Collider {
                    transform: &point,
                    bound: &big_bounds
                }
            ),
            Some(MeterVec2::from_pixel_vec(Vec2::new(-7.0, 0.0))),
        );

        // touching outline, not considered as a collision
        point = Transform::from_xyz(10.0, 0.0, 0.0);
        assert_eq!(
            check_body_collision(
                Collider {
                    transform: &origin,
                    bound: &big_bounds
                },
                Collider {
                    transform: &point,
                    bound: &small_bounds
                }
            ),
            None,
            "Touching outline"
        );
    }

    #[test]
    fn test_projectile_collision() {
        let origin = Transform::default();
        let bounds = BoundingCircle {
            radius: Meter::from_pixel(4.0),
        };

        let collider = Collider {
            transform: &origin,
            bound: &bounds,
        };

        assert_eq!(
            check_projectile_collision(&Transform::from_xyz(0.0, 2.0, 0.0), collider.clone()),
            Some(MeterVec2::from_pixel_vec(Vec2::new(0.0, -2.0)))
        );

        assert_eq!(
            check_projectile_collision(&Transform::from_xyz(0.0, 0.0, 0.0), collider.clone()),
            Some(MeterVec2::from_pixel_vec(Vec2::new(4.0, 0.0)))
        );

        assert_eq!(
            check_projectile_collision(&Transform::from_xyz(0.0, 4.0, 0.0), collider.clone()),
            None
        );

        assert_eq!(
            check_projectile_collision(&Transform::from_xyz(3.6, 3.6, 0.0), collider.clone()),
            None
        );
    }
}
