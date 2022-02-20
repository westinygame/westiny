use crate::components::SpriteId;
use crate::resources::SpriteResource;
use bevy::prelude::{Added, Commands, Entity, Query, Res, TextureAtlasSprite, Visibility, Without};

pub fn add_sprite_to_new_sprite_id(
    mut commands: Commands,
    entities_to_add_sprite: Query<
        (Entity, &SpriteId),
        (Added<SpriteId>, Without<TextureAtlasSprite>),
    >,
    sprite_resource: Res<SpriteResource>,
) {
    entities_to_add_sprite
        .iter()
        .map(|(entity, &sprite_id)| {
            let sprite = sprite_resource.sprite_for(sprite_id);
            (entity, sprite)
        })
        .for_each(|(entity, sprite)| {
            commands
                .entity(entity)
                .insert(sprite)
                .insert(sprite_resource.sprites.clone())
                .insert(Visibility::default());
        });
}
