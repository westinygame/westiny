use crate::states::AppState;
use crate::systems;
use crate::components;
use crate::entities::tilemap::initialize_tilemap;
use bevy::prelude::*;

pub fn setup_system_set() -> SystemSet {
    SystemSet::on_enter(AppState::PlayInit)
        .with_system(systems::build_map)
        .with_system(systems::camera::setup)
        .with_system(systems::hud::setup)
        .with_system(systems::notification_bar::setup)
        .with_system(initialize_tilemap)
}

pub fn init_system_set() -> SystemSet {
    SystemSet::on_update(AppState::PlayInit)
        .with_system(
            systems::receive_network_messages
                .label("network_reception"))
        .with_system(
            systems::spawn_this_player
                .after("network_reception"))
}

pub fn system_set() -> SystemSet {
    SystemSet::on_update(AppState::Play)
        .with_system(
            systems::receive_network_messages
                .label("network_reception"))
        .with_system(
            systems::play_audio)
        .with_system(
            systems::update_network_entities
                .label("update_network_entities")
                .after("network_reception"))
        .with_system(
            systems::camera::follow_player
                .label("camera_follow_player")
                .after("update_network_entities"))
        .with_system(
            systems::handle_user_inputs
                .label("user_input_handler"))
        .with_system(
            systems::spawn_bullets
                .label("shooter")
                .before("physics"))
        .with_system(
            systems::physics
                .label("physics"))
        .with_system(
            systems::update_player
                .label("update_player")
                .after("network_reception"))
        .with_system(
            systems::hud::update_hud
                .after("update_player")
            )
        .with_system(
            systems::hud::update_hud_w
                .after("update_player"))
        .with_system(
            systems::notification_bar::update_notification_bar
                .after("update_player")
            )
}
