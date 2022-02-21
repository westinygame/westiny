use bevy::prelude::*;
use bevy::render::render_resource::TextureUsages;
use bevy_ecs_tilemap::prelude::*;

use westiny_common::metric_dimension::length::Meter;

const TILE_SIZE: Meter = Meter(1.0);
const MAP_SIZE: u32 = 64;
const CHUNK_SIZE: u32 = 16;

pub fn initialize_tilemap(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut map_query: MapQuery)
{

    let texture_handle = asset_server.load("spritesheet.png");

    let map_entity = commands.spawn().id();
    let mut map = Map::new(0u16, map_entity);

    let (mut layer_builder, _) = LayerBuilder::new(
        &mut commands,
        LayerSettings::new(
            MapSize(MAP_SIZE/CHUNK_SIZE, MAP_SIZE/CHUNK_SIZE),
            ChunkSize(CHUNK_SIZE, CHUNK_SIZE),
            TileSize(TILE_SIZE.into_pixel(), TILE_SIZE.into_pixel()),
            TextureSize(32.0, 32.0),
        ),
        0u16,
        0u16,
    );

    layer_builder.set_all(TileBundle::default());

    let layer_entity = map_query.build_layer(&mut commands, layer_builder, texture_handle);
    map.add_layer(&mut commands, 0u16, layer_entity);

    let offset = -((MAP_SIZE/2u32) as f32 * TILE_SIZE.into_pixel());
    commands
        .entity(map_entity)
        .insert(map)
        .insert(Transform::from_xyz(offset - Meter(0.5).into_pixel(), offset + Meter(0.5).into_pixel(), 0.0))
        .insert(GlobalTransform::default());
}

pub fn set_texture_filters_to_nearest(
    mut texture_events: EventReader<AssetEvent<Image>>,
    mut textures: ResMut<Assets<Image>>,
) {
    // quick and dirty, run this for all textures anytime a texture is created.
    for event in texture_events.iter() {
        match event {
            AssetEvent::Created { handle } => {
                if let Some(mut texture) = textures.get_mut(handle) {
                    texture.texture_descriptor.usage = TextureUsages::TEXTURE_BINDING
                        | TextureUsages::COPY_SRC
                        | TextureUsages::COPY_DST;
                }
            }
            _ => (),
        }
    }
}
