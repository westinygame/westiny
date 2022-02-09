use bevy::prelude::SystemSet;
use bevy::core::FixedTimestep;
use crate::systems;
use crate::states::AppState;

pub fn connect_state_systems() -> SystemSet {
    SystemSet::on_update(AppState::Connect)
        .with_run_criteria(FixedTimestep::step(1.0))
        .with_system(systems::connect_to_server)
}

