use crate::components::Velocity;
use crate::metric_dimension::Second;
use bevy::prelude::*;

pub fn physics(time: Res<Time>, mut query: Query<(&mut Transform, &Velocity)>) {
    for (mut transform, velocity) in query.iter_mut() {
        update_position(&mut transform, velocity, Second(time.delta_seconds()))
    }
}

/// Updates transform with velocity based on time
/// Returns delta (x,y) vector
fn update_position(transform: &mut Transform, velocity: &Velocity, delta_time: Second) {
    let delta = delta_time * velocity.0;
    transform.translation += delta.into_pixel_vec().extend(0.0);
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::metric_dimension::{length::Meter, MeterPerSecVec2};

    #[test]
    fn test_update_position() {
        let mut transform =
            Transform::from_xyz(Meter(100.0).into_pixel(), Meter(100.0).into_pixel(), 0.0);

        let velocity = Velocity(MeterPerSecVec2::from_raw(-50.0, -50.0));

        update_position(&mut transform, &velocity, Second(0.5));

        let [x, y, _] = transform.translation.to_array();

        assert_eq!(x, Meter(75.0).into_pixel());
        assert_eq!(y, Meter(75.0).into_pixel());
    }
}
