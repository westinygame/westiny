use amethyst::{
    derive::SystemDesc,
    ecs::{System, SystemData, Read},
    shrev::{ReaderId, EventChannel},
};
use derive_new::new;
use westiny_common::network::EntityState;
use amethyst::core::ecs::{WriteStorage, Join, Entities};
use westiny_common::components::{NetworkId, EntityType};
use amethyst::core::Transform;
use std::collections::HashMap;
use amethyst::shred::ReadExpect;

use crate::resources;
use amethyst::renderer::SpriteRender;

#[derive(SystemDesc, new)]
#[system_desc(name(NetworkEntityStateUpdateSystemDesc))]
pub struct NetworkEntityStateUpdateSystem {
    #[system_desc(event_channel_reader)]
    reader: ReaderId<EntityState>,
}

impl<'s> System<'s> for NetworkEntityStateUpdateSystem {
    type SystemData = (
        Read<'s, EventChannel<EntityState>>,
        WriteStorage<'s, NetworkId>,
        WriteStorage<'s, Transform>,
        WriteStorage<'s, SpriteRender>,
        Entities<'s>,
        ReadExpect<'s, resources::SpriteResource>,
    );

    fn run(&mut self, (events, mut network_ids, mut transforms, mut sprite_renders, entities, sprite_resource): Self::SystemData) {
        let mut entity_states: HashMap<_, _> = events.read(&mut self.reader).map(|entity_state| (entity_state.network_id, entity_state)).collect();

        for (net_id, transform) in (&network_ids, &mut transforms).join() {
            if let Some(state) = entity_states.get(net_id) {
                update_transform(transform, &state);
                entity_states.remove(&net_id);
            }
        }

        for (net_id, entity_state) in entity_states {
            let sprite_id = match net_id.entity_type {
                EntityType::Player => resources::SpriteId::Player,
                EntityType::Bullet => resources::SpriteId::Bullet,
            };

            let transform = {
                let mut transform = Transform::default();
                update_transform(&mut transform, &entity_state);
                transform
            };

            entities.build_entity()
                .with(net_id, &mut network_ids)
                .with(transform, &mut transforms)
                .with(sprite_resource.sprite_render_for(sprite_id), &mut sprite_renders)
                .build();
        }
    }
}

fn update_transform(transform: &mut Transform, entity_state: &EntityState) {
    transform.set_translation_x(entity_state.position.x);
    transform.set_translation_y(entity_state.position.y);
    transform.set_rotation_2d(entity_state.rotation);
}