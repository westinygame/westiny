use bevy::prelude::*;
use bevy::render::render_resource::TextureUsages;
use bevy_ecs_tilemap::prelude::*;
use bevy_ecs_tilemap::helpers::filling::fill_tilemap;

use westiny_common::metric_dimension::length::Meter;

const TILE_SIZE: Meter = Meter(1.0);
const MAP_SIZE: u32 = 64;

pub fn initialize_tilemap(
    mut commands: Commands,
    asset_server: Res<AssetServer>
    )
{
    let texture_handle: Handle<Image> = asset_server.load("spritesheet.png");

    let tilemap_size = TilemapSize{x: MAP_SIZE, y: MAP_SIZE}; // ? /CHUNK_SIZE?

    // Layer 1
    let mut tile_storage = TileStorage::empty(tilemap_size);
    let map_entity = commands.spawn_empty().id();

    fill_tilemap(
        TileTextureIndex(0),
        tilemap_size,
        TilemapId(map_entity),
        &mut commands,
        &mut tile_storage
        );

    let tile_size = TilemapTileSize { x: TILE_SIZE.into_pixel(), y: TILE_SIZE.into_pixel() };
    let grid_size = tile_size.into();
    let map_type = TilemapType::Square;

    commands.entity(map_entity).insert(TilemapBundle {
        grid_size,
        map_type,
        size: tilemap_size,
        storage: tile_storage,
        texture: TilemapTexture::Single(texture_handle),
        tile_size,
        transform: get_tilemap_center_transform(&tilemap_size, &grid_size, &map_type, 0.0),
        ..Default::default()
    });

}

// TODO This is a workaround. Is it still necessary?
pub fn set_texture_filters_to_nearest(
    mut texture_events: EventReader<AssetEvent<Image>>,
    mut textures: ResMut<Assets<Image>>,
) {
    // quick and dirty, run this for all textures anytime a texture is created.
    for event in texture_events.iter() {
        if let AssetEvent::Created { handle } = event {
            if let Some(mut texture) = textures.get_mut(handle) {
                    texture.texture_descriptor.usage = TextureUsages::TEXTURE_BINDING
                        | TextureUsages::COPY_SRC
                        | TextureUsages::COPY_DST;
            }
        }
    }
}
