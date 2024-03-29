pub use audio_player::play_audio;
pub use input_state::handle_user_inputs;
//pub use notification_bar::NotificationBarSystemDesc;
pub use network_entity_delete::delete_entities;
pub use network_entity_update::{update_network_entities, spawn_this_player};
pub use network_messenger::receive_network_messages;
pub use shooter::spawn_bullets;
pub use westiny_common::systems::*;
pub use player_update::update_player;
pub use client_connect::{receive_connection_response, send_connection_request};
pub use sprite::add_sprite_to_new_sprite_id;

mod audio_player;
pub mod hud;
pub mod notification_bar;
mod network_entity_update;
mod network_messenger;
pub mod network_entity_delete;
//mod notification_bar;
pub mod camera;
mod input_state;
mod client_connect;
mod shooter;
mod player_update;
mod sprite;
