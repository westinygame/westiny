
use amethyst::prelude::*;
use amethyst::core::Transform;

use log::info;
use crate::components::{Player, Velocity};
use crate::resources::SpriteResource;
use amethyst::core::math::Point2;

pub fn initialize_player(world: &mut World,
                         sprite_resource: &SpriteResource,
                         start_pos: Point2<f32>
                         ) {

    let mut transform = Transform::default();
    transform.set_translation_xyz(start_pos.x, start_pos.y, 0.0);

    world
        .create_entity()
        .with(sprite_resource.sprite_render_for_player())
        .with(transform)
        .with(Player)
        .with(Velocity::default())
        .build();

    info!("Player created.");
}
