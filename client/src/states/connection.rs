use crate::states::AppState;
use crate::systems;
use bevy::core::FixedTimestep;
use bevy::prelude::SystemSet;
use bevy::prelude::ParallelSystemDescriptorCoercion;

pub fn connect_state_systems() -> SystemSet {
    SystemSet::on_update(AppState::Connect)
        .with_system(systems::send_connection_request
                     .label("connection_request"))
        .with_system(systems::receive_connection_response
                     .label("connection_response"))
}
