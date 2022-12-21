use serde::Deserialize;
use std::io::Read;

pub fn read_ron<T>(ron_path: &std::path::Path) -> anyhow::Result<T>
where
    T: for<'a> Deserialize<'a>,
{
    let content = {
        let mut file = std::fs::File::open(ron_path)?;
        let mut buffer = Vec::new();
        file.read_to_end(&mut buffer)?;

        buffer
    };

    let mut de = ron::de::Deserializer::from_bytes(&content)?;
    let deserialized = T::deserialize(&mut de)?;
    de.end()?;
    Ok(deserialized)
}

pub fn rotate_toward_point(
    transform: &mut bevy::transform::components::Transform,
    vector: &bevy::math::Vec2,
) {
    let dir_vec = *vector - transform.translation.truncate();
    transform.rotation = get_rotation(&dir_vec);
}

pub fn get_rotation(vector: &bevy::math::Vec2) -> bevy::math::Quat {
    use bevy::math::{Quat, Vec2};
    Quat::from_rotation_arc_2d(Vec2::NEG_Y, vector.normalize())
}

pub fn get_angle(quat: bevy::math::Quat) -> f32 {
    let (axis, angle) = quat.to_axis_angle();
    if axis.z < 0.0 {
        std::f32::consts::PI*2.0 - angle
    } else {
        angle
    }
}

pub fn rotate_vec3_around_z(quat: bevy::math::Quat, vec: &mut bevy::math::Vec3) {
    *vec = quat.mul_vec3(*vec);
}

#[cfg(test)]
mod test {
    use std::f32::consts::PI;
    use westiny_test::*;

    const FACING_UP: f32 = PI;
    const FACING_DOWN: f32 = 0.0;
    const FACING_LEFT: f32 = -PI / 2.0;
    const FACING_RIGHT: f32 = PI / 2.0;

    mod test_rotate_toward_point {
        use super::*;
        use crate::utilities::*;
        use bevy::math::Vec2;
        use bevy::prelude::Transform;

        macro_rules! test_rotate_toward_point {
            ($($name:ident: $vector_coord:expr, $expected:expr,)*) => {
                $(
                    #[test]
                    fn $name() {
                        let mut transform = &mut Transform::default();

                        let ref_vector = Vec2::new($vector_coord.0, $vector_coord.1);

                        rotate_toward_point(&mut transform, &ref_vector);

                        let angle = get_angle(transform.rotation);
                        // sin is called to normalize the angles (e.g. -PI = PI)
                        assert_delta!(f32::sin($expected), angle.sin(), 0.0001)
                    }
                )*
            }
        }

        test_rotate_toward_point! {
            rotate_up: (0.0, 2.0), FACING_UP,
            rotate_left: (-2.0, 0.0), FACING_LEFT,
            rotate_down: (0.0, -4.0), FACING_DOWN,
            rotate_right: (4.0, 0.0), FACING_RIGHT,
            rotate_upright_45deg: (4.0, 4.0), 3.0*PI/4.0,
            rotate_downright_30deg: (0.866025404*0.001, -0.001/2.0), 1.04719755,
        }
    }
    
    #[test]
    fn test_get_angle() {
        use bevy::math::{Vec3, Quat};

        let angle_to_quat = | angle: f32 | -> Quat {
            Quat::from_axis_angle(Vec3::Z, angle)
        };
        
        for i in 0..(2.0 * 100.0*PI) as usize {
            let angle = i as f32 / 100.0;
            let quat = angle_to_quat(angle);
            let actual = crate::utilities::get_angle(quat);
            assert_delta!(angle, actual, 0.0001);
        }
    }
}
