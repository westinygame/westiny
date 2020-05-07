
use amethyst::assets::Handle;
use amethyst::prelude::*;
use amethyst::renderer::{SpriteSheet, SpriteRender};
use amethyst::core::Transform;

use log::info;
use crate::components::{Player, Velocity};
use amethyst::core::math::Point2;

pub fn initialize_player(world: &mut World,
                         sprite_sheet_handle: Handle<SpriteSheet>,
                         start_pos: Point2<f32>
                         ) {

    let mut transform = Transform::default();
    transform.set_translation_xyz(start_pos.x, start_pos.y, 0.0);

    let sprite_render = SpriteRender {
        sprite_sheet: sprite_sheet_handle,
        sprite_number: 3,
    };

    world
        .create_entity()
        .with(sprite_render.clone())
        .with(transform)
        .with(Player)
        .with(Velocity::default())
        .build();

    info!("Player created.");
}
