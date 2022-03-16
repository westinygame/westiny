use crate::components::{Lifespan, Projectile, Velocity};
use crate::metric_dimension::{length::MeterVec2, MeterPerSecVec2, Second};
use bevy::prelude::{Transform, GlobalTransform, Bundle};
use std::time::Duration;

#[derive(Bundle)]
pub struct BulletBundle {
    #[bundle]
    pub transform: (Transform, GlobalTransform),

    pub velocity: Velocity,
    pub projectile: Projectile,
    pub lifespan: Lifespan,
}

impl BulletBundle {
    pub fn new(
        position: MeterVec2,
        velocity: MeterPerSecVec2,
        lives_for_seconds: Second,
        current_time: Duration,
    ) -> Self {
        let transform = {
            let mut transform = Transform::from_xyz(
                position.x.into_pixel(),
                position.y.into_pixel(),
                1.0);
            transform.rotation = crate::utilities::get_rotation(&velocity.into_pixel_per_sec_vec());
            transform
        };

        BulletBundle {
            transform: (transform, GlobalTransform::default()),
            velocity: Velocity(velocity),
            projectile: Projectile,
            lifespan: Lifespan::new(lives_for_seconds, current_time),
        }
    }
}
