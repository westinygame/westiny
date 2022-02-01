pub use client_introduction::introduce_new_clients;
pub use command_transformer::transform_commands;
pub use entity_delete_broadcaster::entity_delete_system_set;
pub use entity_state_broadcaster::broadcast_entity_state;
pub use health::handle_damage;
pub use network_messenger::read_network_messages;
pub use player_movement::apply_input;
pub use shooter::weapon_handler_system_set;
// pub use spawn::{SpawnPlayerEvent, SpawnSystemDesc, RespawnSystem};
pub use death::handle_death;
pub use spawn::{spawn_player, SpawnPlayerEvent};
pub use westiny_common::systems::*;

mod client_introduction;
mod command_transformer;
mod death;
mod entity_delete_broadcaster;
mod entity_state_broadcaster;
mod health;
mod network_messenger;
mod player_movement;
mod shooter;
mod spawn;

use crate::resources;
use bevy::prelude::{Commands, Res};

pub fn build_map(
    commands: Commands,
    seed: Res<resources::Seed>,
    res_dir: Res<resources::ResourcesDir>,
) {
    let res = resources::map::build_map(commands, *seed, &res_dir.0.join("map"));
    match res {
        Ok(()) => bevy::log::info!("Map built"),
        Err(err) => bevy::log::error!("{}", err),
    };
}
