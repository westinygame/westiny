
use amethyst::prelude::*;
use amethyst::core::Transform;

use log::info;
use crate::components::{Player, Velocity, Weapon, WeaponDetails, weapon::Shot};
use crate::resources::{SpriteResource, SpriteId};
use amethyst::core::math::Point2;

pub fn initialize_player(world: &mut World,
                         sprite_resource: &SpriteResource,
                         start_pos: Point2<f32>
                         ) {

    let mut transform = Transform::default();
    transform.set_translation_xyz(start_pos.x, start_pos.y, 0.0);

    // TODO define these values in RON resource files.
    let revolver = WeaponDetails {
        damage: 5.0,
        distance: 120.0,
        fire_rate: 7.2,
        magazine_size: 6,
        reload_time: 1.0,
        spread: 2.0,
        shot: Shot::Single,
        bullet_speed: 200.0
    };

    world
        .create_entity()
        .with(sprite_resource.sprite_render_for(SpriteId::Player))
        .with(transform)
        .with(Player)
        .with(Velocity::default())
        .with(Weapon::new(revolver))
        .build();

    info!("Player created.");
}
