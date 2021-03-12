use amethyst::{
    derive::SystemDesc,
    ecs::{System, SystemData, Read},
    shrev::{ReaderId, EventChannel},
    core::math::Point2,
    prelude::Builder
};
use derive_new::new;
use westiny_common::network::EntityState;
use amethyst::core::ecs::{ReadStorage, WriteStorage, Join, Entities, WriteExpect, LazyUpdate, world::LazyBuilder};
use westiny_common::components::{NetworkId, EntityType};
use westiny_common::resources::SpriteId;
use amethyst::core::Transform;
use std::collections::HashMap;
use amethyst::shred::ReadExpect;

use crate::resources;
use crate::entities::{create_player, create_character};

#[derive(SystemDesc, new)]
#[system_desc(name(NetworkEntityStateUpdateSystemDesc))]
pub struct NetworkEntityStateUpdateSystem {
    #[system_desc(event_channel_reader)]
    reader: ReaderId<Vec<EntityState>>,
}

impl<'s> System<'s> for NetworkEntityStateUpdateSystem {
    type SystemData = (
        Read<'s, EventChannel<Vec<EntityState>>>,
        ReadStorage<'s, NetworkId>,
        WriteStorage<'s, Transform>,
        Entities<'s>,
        ReadExpect<'s, resources::SpriteResource>,
        ReadExpect<'s, resources::PlayerNetworkId>,
        Read<'s, LazyUpdate>,
    );

    fn run(&mut self,
           (
               events,
               network_ids,
               mut transforms,
               entities,
               sprite_resource,
               player_net_id,
               lazy,
           ): Self::SystemData) {
        let mut entity_states: HashMap<_, _> = events.read(&mut self.reader).flat_map(|vec| vec.iter()).map(|entity_state| (entity_state.network_id, entity_state)).collect();

        for (net_id, transform) in (&network_ids, &mut transforms).join() {
            if let Some(state) = entity_states.get(net_id) {
                update_transform(transform, &state);
                entity_states.remove(&net_id);
            }
        }

        // if it is this player
        if let Some(&new_state) = entity_states.get(&player_net_id.0) {
            create_player(||{ lazy.create_entity(&entities) }, &sprite_resource, player_net_id.0, as_transform(&new_state.position));
            entity_states.remove(&player_net_id.0);
        }


        for (net_id, entity_state) in entity_states {
            let mut transform = Transform::default();
            update_transform(&mut transform, &entity_state);

            match net_id.entity_type {
                EntityType::Player => {
                        create_character(lazy.create_entity(&entities), ||{ lazy.create_entity(&entities)}, &sprite_resource, net_id, transform);
                }
                EntityType::Corpse => {
                    // TODO constants should be used instead of magic numbers
                    transform.set_translation_z(-0.9);
                    spawn_entity(lazy.create_entity(&entities), net_id, transform, &sprite_resource, SpriteId::Corpse);
                },
            };
        }
    }
}

fn spawn_entity(
    builder: LazyBuilder,
    net_id: NetworkId,
    transform: Transform,
    sprite_resource: &resources::SpriteResource,
    sprite_id: SpriteId)
{
    builder
        .with(net_id)
        .with(transform)
        .with(sprite_resource.sprite_render_for(sprite_id))
        .build();
}


fn update_transform(transform: &mut Transform, entity_state: &EntityState) {
    transform.set_translation_x(entity_state.position.x);
    transform.set_translation_y(entity_state.position.y);
    transform.set_rotation_2d(entity_state.rotation);
}

fn as_transform(pos: &Point2<f32>) -> Transform
{
    let mut transform = Transform::default();
    transform.set_translation_xyz(pos.x, pos.y, 0.0);
    transform
}

