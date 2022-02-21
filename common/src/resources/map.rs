use crate::entities;
use crate::resources::map::MapError::{InvalidMapCharacter, SeedError};
use crate::resources::Seed;
use bevy::prelude::{Commands, Vec2};
use std::fmt::{Display, Formatter};
use std::fs::File;
use std::io::{BufReader, Read};
use std::path::{Path, PathBuf};
use std::result::Result;

const BARREL_CHAR: char = 'x';
const EMPTY_CHAR: char = ' ';

const MAP_OFFSET: (i32, i32) = (-32, -32);

pub fn build_map(mut commands: Commands, seed: Seed, map_files_dir: &Path) -> Result<(), MapError> {
    if seed.0 == 0 {
        let map_file_path = map_files_dir.join("rust2.wmap");
        let open_file_result = File::open(&map_file_path);
        if let Err(err) = open_file_result {
            return Err(MapError::MapFileError(map_file_path, err));
        }

        let map_bytes = BufReader::new(open_file_result.unwrap()).bytes();

        let mut x = 0;
        let mut y = 0;
        for byte in map_bytes {
            if let Err(err) = byte {
                return Err(MapError::MapFileError(map_file_path, err));
            }
            match byte.unwrap() as char {
                BARREL_CHAR => {
                    // spawn a barrel
                    let pos = Vec2::new((x + MAP_OFFSET.0) as f32, -(y + MAP_OFFSET.1) as f32);
                    entities::place_barrel(&mut commands, pos);
                    x += 1;
                }
                EMPTY_CHAR => {
                    // spawn nothing
                    x += 1;
                }
                '\n' => {
                    // just step to next row
                    x = 0;
                    y += 1;
                }
                other => return Err(InvalidMapCharacter(other, x, y)),
            }
        }

        Ok(())
    } else {
        Err(SeedError(seed))
    }
}

#[derive(Debug)]
pub enum MapError {
    InvalidMapCharacter(char, i32, i32),
    MapFileError(PathBuf, std::io::Error),
    SeedError(Seed),
}

impl Display for MapError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let literal = match self {
            Self::InvalidMapCharacter(ch, x, y) => {
                format!("Could not process char ({}) at ({}, {})", ch, x, y)
            }
            Self::MapFileError(path, inner) => format!(
                "File IO error: {} (path: {})",
                inner,
                path.to_str().unwrap()
            ),
            Self::SeedError(seed) => format!("Could not handle seed: {}", seed),
        };
        write!(f, "{}", literal)
    }
}

impl std::error::Error for MapError {}
