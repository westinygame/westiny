use crate::components::{Velocity};
use crate::metric_dimension::Second;
use bevy::prelude::*;

pub fn physics(time: Res<Time>,
               mut query: Query<(&mut Transform, &Velocity)>) {
    for (mut transform, velocity) in query.iter_mut() {
        update_position(&mut transform, &velocity, &time)
    }
}

/// Updates transform with velocity based on time
/// Returns delta (x,y) vector
fn update_position(transform: &mut Transform, velocity: &Velocity, time: &Time) {
    let delta = Second(time.delta_seconds()) * velocity.0;
    transform.translation += delta.into_pixel_vec().extend(0.0);
}

#[cfg(test)]
mod test {
    use super::*;
    use amethyst::core::math::Vector2;
    use crate::metric_dimension::MeterPerSec;

    #[test]
    fn test_update_position() {
        let mut transform = Transform::default();
        transform.set_translation_x(Meter(100.0).into_pixel());
        transform.set_translation_y(Meter(100.0).into_pixel());

        let velocity = Velocity(Vector2::new(MeterPerSec(-50.0), MeterPerSec(-50.0)));
        let mut time = Time::default();
        time.set_delta_seconds(0.5);

        update_position(&mut transform, &velocity, &time);

        assert_eq!(transform.translation().x.round(), Meter(75.0).into_pixel());
        assert_eq!(transform.translation().y.round(), Meter(75.0).into_pixel());
    }
}
