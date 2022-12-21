use crate::components::{EntityType, NetworkId};
use crate::entities::{create_player_character, create_this_player, CorpseBundle};
use crate::resources::PlayerNetworkId;
use crate::states::AppState;
use std::collections::HashMap;
use westiny_common::metric_dimension::length::Meter;
use westiny_common::network::{EntityState, PlayerDeath};

use bevy::prelude::*;

pub fn spawn_this_player(
    mut commands: Commands,
    mut entity_states_events: EventReader<Vec<EntityState>>,
    player_net_id: Res<PlayerNetworkId>,
    mut app_state: ResMut<State<AppState>>
) {
    let maybe_this_player = entity_states_events
        .iter()
        .flat_map(|vec| vec.iter())
        .filter(|state| state.network_id == player_net_id.0)
        .last();

    if let Some(entity_state) = maybe_this_player {
        create_this_player(
            &mut commands,
            player_net_id.0,
            entity_state.position.into_transform(Meter(0.0)),
        );
        log::debug!("Player spawned at {:?}", entity_state.position);
        app_state.set(AppState::Play)
            .expect("Unable to set app state to Play");
    }
}
pub fn update_network_entities(
    mut commands: Commands,
    mut entity_states_events: EventReader<Vec<EntityState>>,
    mut player_death: EventReader<PlayerDeath>,
    mut network_transforms: Query<(&NetworkId, &mut Transform)>,
    time: Res<Time>,
) {
    let mut entity_states: HashMap<_, _> = entity_states_events
        .iter()
        .flat_map(|vec| vec.iter())
        .map(|entity_state| (entity_state.network_id, entity_state))
        .collect();

    for (net_id, mut transform) in network_transforms.iter_mut() {
        if let Some(state) = entity_states.get(net_id) {
            update_transform(&mut transform, state);
            entity_states.remove(net_id);
        }
    }

    for (net_id, entity_state) in entity_states {
        let mut transform = Transform::default();
        update_transform(&mut transform, entity_state);

        // Yeah it looks silly but there will be more network entities
        match net_id.entity_type {
            EntityType::Player => create_player_character(&mut commands, net_id, transform),
        };
    }

    player_death
        .iter()
        .map(|death| death.position.into_transform(Meter(0.0)))
        .for_each(|transform| {
            commands.spawn(CorpseBundle::new(transform, time.elapsed()));
        });
}

fn update_transform(transform: &mut Transform, entity_state: &EntityState) {
    transform.translation.x = entity_state.position.x.into_pixel();
    transform.translation.y = entity_state.position.y.into_pixel();
    transform.rotation = Quat::from_rotation_z(entity_state.angle);
}
