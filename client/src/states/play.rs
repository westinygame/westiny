use crate::states::AppState;
use crate::systems;
use bevy::prelude::{ParallelSystemDescriptorCoercion, SystemSet};

pub fn setup_system_set() -> SystemSet {
    SystemSet::on_enter(AppState::Play)
        .with_system(systems::build_map)
        .with_system(systems::camera::setup)
}

pub fn system_set() -> SystemSet {
    SystemSet::on_update(AppState::Play)
        .with_system(systems::receive_network_messages.label("network_reception"))
        .with_system(
            systems::update_network_entities
                .label("update_network_entities")
                .after("network_reception"),
        )
        .with_system(
            systems::camera::follow_player
                .label("camera_follow_player")
                .after("update_network_entities"),
        )
}
