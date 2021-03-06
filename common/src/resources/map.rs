use amethyst::core::ecs::{World, Entity};
use std::result::Result;
use std::fs::File;
use std::fmt::{Display, Formatter};
use std::path::Path;
use std::io::{BufReader, Read};
use crate::resources::map::MapError::{InvalidMapCharacter, SeedError};
use crate::resources::{SpriteId, Seed};
use amethyst::core::math::Point2;

const BARREL_CHAR: char = 'x';
const EMPTY_CHAR: char = ' ';

const MAP_OFFSET: (i32, i32) = (-32, -32);

pub fn build_map(world: &mut World,
                 seed: Seed,
                 map_files_dir: &Path) -> Result<Vec<(Entity, SpriteId)>, MapError> {
    let mut entity_vec = Vec::new();
    if seed.0 == 0 {
        let map_reader = BufReader::new(File::open(map_files_dir.join("rust2.wmap"))?);
        let map_bytes = map_reader.bytes();

        let mut x = 0;
        let mut y = 0;
        for byte in map_bytes {
            match byte? as char {
                BARREL_CHAR => {
                    // spawn a barrel
                    let pos = Point2::new(x + MAP_OFFSET.0, -(y + MAP_OFFSET.1));
                    let barrel = crate::entities::place_barrel(world, pos);
                    entity_vec.push((barrel, SpriteId::Barrel));
                    x += 1;
                },
                EMPTY_CHAR => {
                    // spawn nothing
                    x += 1;
                },
                '\n' => {
                    // just step to next row
                    x = 0;
                    y += 1;
                }
                other => return Err(InvalidMapCharacter(other, x, y))
            }

        }

        Ok(entity_vec)
    } else {
        Err(SeedError(seed))
    }
}

#[derive(Debug)]
pub enum MapError {
    InvalidMapCharacter(char, i32, i32),
    MapFileError(std::io::Error),
    SeedError(Seed),
}

impl Display for MapError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let literal = match self {
            Self::InvalidMapCharacter(ch, x, y) => format!("Could not process char ({}) at ({}, {})", ch, x, y),
            Self::MapFileError(inner) => format!("File IO error: {}", inner),
            Self::SeedError(seed) => format!("Could not handle seed: {}", seed),
        };
        write!(f, "{}", literal)
    }
}

impl std::error::Error for MapError {}

impl From<std::io::Error> for MapError {
    fn from(err: std::io::Error) -> Self {
        MapError::MapFileError(err)
    }
}