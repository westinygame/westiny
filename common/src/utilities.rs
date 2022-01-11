use serde::Deserialize;
use std::io::Read;

pub fn read_ron<T>(ron_path: &std::path::Path) -> anyhow::Result<T>
    where T: for<'a> Deserialize<'a> {

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

pub fn set_rotation_toward_vector(transform: &mut bevy::transform::components::Transform, vector: &bevy::math::Vec2) {
    use bevy::math::{Vec2, Vec3, Quat};
    let dir_vec = *vector - transform.translation.truncate();
    let angle = {
        let abs_angle = dir_vec.angle_between(Vec2::new(0.0, -1.0));
        if dir_vec.x < 0.0
        {
            2.0 * std::f32::consts::PI - abs_angle
        } else {
            abs_angle
        }
    };
    transform.rotation = Quat::from_axis_angle(Vec3::Z, angle);
}

pub fn rotate_vec3_around_z(quat: &bevy::math::Quat, vec: &mut bevy::math::Vec3) {
    *vec = quat.mul_vec3(*vec);
    if quat.z < 0.0 {
        vec.x = -vec.x;
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use westiny_test::f32_eq;
    use std::f32::consts::PI;

    const FACING_UP: f32 = PI;
    const FACING_DOWN: f32 = 0.0;
    const FACING_LEFT: f32 = -PI/2.0;
    const FACING_RIGHT: f32 = PI/2.0;

    mod test_set_rotation_toward_vector {
        use super::*;

        macro_rules! test_set_rotation_toward_vector {
            ($($name:ident: $vector_coord:expr, $expected:expr,)*) => {
                $(
                    #[test]
                    fn $name() {
                        let mut transform = &mut Transform::default();

                        let ref_vector = Vector2::new($vector_coord.0, $vector_coord.1);

                        set_rotation_toward_vector(&mut transform, &ref_vector);

                        let angle = transform.rotation().axis().map(|vec| vec.z).unwrap_or(1.0) * transform.rotation().angle();
                        // sin is called to normalize the angles (e.g. -PI = PI)
                        assert!(f32_eq(f32::sin($expected), angle.sin()), "Expected angle: {}, Actual angle: {}", $expected, angle);
                    }
                )*
            }
        }

        test_set_rotation_toward_vector! {
            rotate_up: (0.0, 2.0), FACING_UP,
            rotate_left: (-2.0, 0.0), FACING_LEFT,
            rotate_down: (0.0, -4.0), FACING_DOWN,
            rotate_right: (4.0, 0.0), FACING_RIGHT,
            rotate_upright_45deg: (4.0, 4.0), 3.0*PI/4.0,
            rotate_downright_30deg: (0.866025404*0.001, -0.001/2.0), 1.04719755,
        }
    }
}
