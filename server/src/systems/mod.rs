pub use client_introduction::introduce_new_clients;
pub use command_transformer::transform_commands;
// pub use entity_delete_broadcaster::EntityDeleteBroadcasterSystemDesc;
pub use entity_state_broadcaster::broadcast_entity_state;
// pub use health::HealthSystemDesc;
pub use network_messenger::read_network_messages;
pub use player_movement::apply_input;
// pub use shooter::ShooterSystem;
// pub use spawn::{SpawnPlayerEvent, SpawnSystemDesc, RespawnSystem};
pub use spawn::{SpawnPlayerEvent, spawn_player};
// pub use death::DeathSystem;
pub use westiny_common::systems::*;

mod network_messenger;
mod client_introduction;
mod command_transformer;
// mod entity_delete_broadcaster;
mod entity_state_broadcaster;
// mod shooter;
mod player_movement;
// mod health;
mod spawn;
// mod death;

use bevy::prelude::{Commands, Res};
use crate::resources;

pub fn build_map(commands: Commands,
                 seed: Res<resources::Seed>,
                 res_dir: Res<resources::ResourcesDir>) {
    let res = resources::map::build_map(commands, *seed, &res_dir.0.join("map"));
    match res {
        Ok(()) => bevy::log::info!("Map built"),
        Err(err) => bevy::log::error!("{}", err)
    };
}

