use amethyst::core::Transform;
use amethyst::core::transform::Parent;
use amethyst::prelude::*;
use amethyst::ecs::Entity;
use log::info;

use westiny_common::components::{Input, Health, Player, NetworkId, BoundingCircle};
use crate::resources::SpriteResource;
use westiny_common::resources::SpriteId;
use westiny_common::components::weapon::Weapon;
use westiny_common::resources::weapon::{GunResource, GunId};

pub const CHARACTER_HEIGHT : f32 = 1.8;

pub fn create_hand_for_character<B: Builder>(
    builder: B,
    sprite_resource: &SpriteResource,
    parent: Entity
    ) -> Entity
{
    let mut hand_transform = Transform::default();
    hand_transform.set_translation_xyz(-3., -6., -0.3); // relative to parent

    builder
        .with(Parent{entity: parent})
        .with(hand_transform)
        .with(sprite_resource.sprite_render_for(SpriteId::HandWithPistol))
        .build()
}

pub fn create_character<B: Builder, F: Fn() -> B>(
    character_builder: B,
    factory: F,
    sprite_resource: &SpriteResource,
    network_id: NetworkId,
    mut transform: Transform
    ) -> Entity
{
    transform.set_translation_z(CHARACTER_HEIGHT);
    let entity = character_builder
        .with(network_id)
        .with(sprite_resource.sprite_render_for(SpriteId::Player))
        .with(transform)
        .with(BoundingCircle{radius: 8.0})
        .build();

    create_hand_for_character(factory(), &sprite_resource, entity);
    entity
}

pub fn create_player<B: Builder, F>(
    factory: F,
    sprite_resource: &SpriteResource,
    network_id: NetworkId,
    transform: Transform,
    gun_resource: &GunResource,
    ) -> Entity
where F: Fn () -> B
{
    let builder = factory()
        .with(Player)
        .with(Health(100))
        .with(Input::default())
        .with(Weapon::new(gun_resource.get_gun(GunId::Revolver)));
    let entity = create_character(builder, factory, sprite_resource, network_id, transform);
    info!("Player created.");
    entity
}
