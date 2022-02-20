use crate::components::SpriteId;
use crate::resources;
use bevy::prelude::*;
use serde::Deserialize;
use westiny_common::utilities::read_ron;

#[derive(Clone)]
pub struct SpriteResource {
    pub sprites: Handle<TextureAtlas>,
}

impl Default for SpriteResource {
    fn default() -> Self {
        SpriteResource {
            sprites: Handle::<TextureAtlas>::default(),
        }
    }
}

impl SpriteResource {
    pub fn sprite_for(&self, id: SpriteId) -> TextureAtlasSprite {
        self.sprite_at_index(id as usize)
    }

    fn sprite_at_index(&self, index: usize) -> TextureAtlasSprite {
        TextureAtlasSprite {
            index,
            ..Default::default()
        }
    }
}

#[derive(Deserialize)]
pub struct SpriteRect {
    pub x: f32,
    pub y: f32,
    pub width: f32,
    pub height: f32,
}

#[derive(Deserialize)]
pub struct SpriteManifest {
    pub texture_width: f32,
    pub texture_height: f32,
    pub sprites: Vec<SpriteRect>,
}

pub fn initialize_sprite_resource(
    mut sprite_resource: ResMut<SpriteResource>,
    mut texture_atlases: ResMut<Assets<TextureAtlas>>,
    asset_server: Res<AssetServer>,
    res: Res<resources::ResourcesDir>,
) {
    let texture_handle = asset_server.load(res.crate_resources.join("spritesheet.png"));

    let manifest =
        read_ron::<SpriteManifest>(&res.crate_resources.join("spritesheet.ron")).unwrap();

    let mut texture_atlas = TextureAtlas::new_empty(
        texture_handle,
        Vec2::new(manifest.texture_width, manifest.texture_height),
    );

    manifest
        .sprites
        .iter()
        .map(|rect| bevy::sprite::Rect {
            min: Vec2::new(rect.x, rect.y),
            max: Vec2::new(rect.x + rect.width, rect.y + rect.height),
        })
        .for_each(|rect| {
            texture_atlas.add_texture(rect);
        });

    sprite_resource.sprites = texture_atlases.add(texture_atlas);
}
