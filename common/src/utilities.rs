use serde::Deserialize;
use std::io::Read;
use amethyst::core::Transform;
use amethyst::core::math::Vector2;

pub fn read_ron<T>(ron_path: & std::path::Path) -> std::result::Result<T, Box<dyn std::error::Error>>
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

pub fn set_rotation_toward_vector(transform: &mut Transform, vector: &Vector2<f32>) {
    let mut angle = Vector2::new(0.0, -1.0).angle(vector);
    if vector.x < 0.0 {
        angle = 2.0 * std::f32::consts::PI - angle;
    }
    transform.set_rotation_2d(angle);
}
