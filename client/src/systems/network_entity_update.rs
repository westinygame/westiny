use amethyst::{
    derive::SystemDesc,
    ecs::{System, SystemData, Read},
    shrev::{ReaderId, EventChannel},
    core::math::Point2,
    prelude::Builder
};
use derive_new::new;
use amethyst::core::ecs::{ReadStorage, WriteStorage, Join, Entities, LazyUpdate, world::LazyBuilder};
use westiny_common::network::{EntityState, PlayerDeath};
use westiny_common::components::{NetworkId, EntityType, Lifespan};
use westiny_common::resources::SpriteId;
use amethyst::core::{Transform, Time};
use std::collections::HashMap;
use std::time::Duration;
use amethyst::shred::ReadExpect;

use crate::resources;
use crate::entities::{create_player, create_character};
use crate::resources::SpriteResource;

const CORPSE_HEIGHT: f32 = 0.1;

#[derive(SystemDesc, new)]
#[system_desc(name(NetworkEntityStateUpdateSystemDesc))]
pub struct NetworkEntityStateUpdateSystem {
    #[system_desc(event_channel_reader)]
    entity_state_reader: ReaderId<Vec<EntityState>>,

    #[system_desc(event_channel_reader)]
    death_reader: ReaderId<PlayerDeath>,
}

impl<'s> System<'s> for NetworkEntityStateUpdateSystem {
    type SystemData = (
        Read<'s, EventChannel<Vec<EntityState>>>,
        Read<'s, EventChannel<PlayerDeath>>,
        ReadStorage<'s, NetworkId>,
        WriteStorage<'s, Transform>,
        Entities<'s>,
        ReadExpect<'s, resources::SpriteResource>,
        ReadExpect<'s, resources::PlayerNetworkId>,
        ReadExpect<'s, LazyUpdate>,
        ReadExpect<'s, Time>,
    );

    fn run(&mut self,
           (
               entity_state_event_channel,
               death_event_channel,
               network_ids,
               mut transforms,
               entities,
               sprite_resource,
               player_net_id,
               lazy,
               time,
           ): Self::SystemData) {
        let mut entity_states: HashMap<_, _> = entity_state_event_channel.read(&mut self.entity_state_reader)
            .flat_map(|vec| vec.iter())
            .map(|entity_state| (entity_state.network_id, entity_state))
            .collect();

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


            // Yeah it looks silly but there will be more network entities
            match net_id.entity_type {
                EntityType::Player => create_character(lazy.create_entity(&entities), ||{ lazy.create_entity(&entities)}, &sprite_resource, net_id, transform)
            };
        }

        let deaths = death_event_channel.read(&mut self.death_reader);
        if let Err(error) = Self::handle_deaths(deaths, &lazy, &sprite_resource, &entities, time.absolute_time()) {
            log::error!("Failed to handle player death event: {}", error);
        }
    }
}

impl<'s> NetworkEntityStateUpdateSystem {
    fn handle_deaths<D: IntoIterator<Item=&'s PlayerDeath>>(
        deaths: D,
        lazy: &LazyUpdate,
        sprite_resource: &SpriteResource,
        entities: &Entities<'_>,
        current_time: Duration,
    ) -> anyhow::Result<()> {
        deaths.into_iter().for_each(|death| {
            let transform = {
                let mut transform = Transform::default();
                transform.set_translation_xyz(death.position.x, death.position.y, -0.9);
                transform
            };
            lazy.create_entity(entities)
                .with(transform)
                .with(sprite_resource.sprite_render_for(SpriteId::Corpse))
                .with(Lifespan::new(60.0, current_time))
                .build();
        });
        Ok(())
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

