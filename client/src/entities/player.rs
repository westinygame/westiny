use amethyst::core::math::Point2;
use amethyst::core::Transform;
use amethyst::prelude::*;
use amethyst::ecs::Entity;
use log::info;

use westiny_common::components::{Input, Health, Player, NetworkId};
use crate::resources::SpriteResource;
use westiny_common::resources::SpriteId;

pub fn initialize_player<B: Builder>(builder: B,
                         sprite_resource: &SpriteResource,
                         network_id: NetworkId,
                         start_pos: Point2<f32>
                         ) -> Entity {

    let mut transform = Transform::default();
    transform.set_translation_xyz(start_pos.x, start_pos.y, 0.0);

    let entity = builder
        .with(network_id)
        .with(sprite_resource.sprite_render_for(SpriteId::Player))
        .with(transform)
        .with(Player)
        .with(Health(100))
        .with(Input::default())
        .build();

    info!("Player created.");

    entity
}
