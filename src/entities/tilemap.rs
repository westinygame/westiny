use amethyst::prelude::*;
use amethyst::core::math::{Point2, Vector3};
use amethyst::core::Transform;
use amethyst::tiles::{TileMap, MortonEncoder};

use westiny_client::resources::SpriteResource;
use crate::resources::GroundTile;

const TILE_SIZE: u32 = 16;
const CHUNK_SIZE: u32 = 64;

pub fn initialize_tilemap(
    world: &mut World,
    sprite_resource: &SpriteResource,
    position: Point2<f32>)
{
    let map = TileMap::<GroundTile, MortonEncoder>::new(
        /*dimensions:*/ Vector3::new(CHUNK_SIZE, CHUNK_SIZE, 1),
        /*tile_dimensions:*/ Vector3::new(TILE_SIZE, TILE_SIZE, 1),
        /*sprite_sheet:*/ Some(sprite_resource.sheet.clone()),
        );

    let mut transform = Transform::default();
    transform.set_translation_xyz(position.x, position.y, -0.9);

    let _map_entity = world
        .create_entity()
        .with(map)
        .with(transform)
        .build();
}
