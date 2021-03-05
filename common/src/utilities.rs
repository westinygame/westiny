use serde::Deserialize;
use std::io::Read;

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
