use serde::Deserialize;
use std::io::Read;
use amethyst::core::Transform;
use amethyst::core::math::Vector2;
use num_traits::ToPrimitive;
use std::fmt::Debug;

pub fn read_ron<T>(ron_path: & std::path::Path) -> anyhow::Result<T>
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

pub fn set_rotation_toward_vector<T>(transform: &mut Transform, vector: &Vector2<T>) where T: 'static + ToPrimitive + Copy + PartialEq + Debug {
    let primitive_vec = Vector2::new(vector.x.to_f32().unwrap(), vector.y.to_f32().unwrap());
    let mut angle = Vector2::new(0.0, -1.0).angle(&primitive_vec);
    if primitive_vec.x < 0.0 {
        angle = 2.0 * std::f32::consts::PI - angle;
    }
    transform.set_rotation_2d(angle);
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