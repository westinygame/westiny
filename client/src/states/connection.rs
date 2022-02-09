use crate::states::AppState;
use crate::systems;
use bevy::core::FixedTimestep;
use bevy::prelude::SystemSet;

pub fn connect_state_systems() -> SystemSet {
    SystemSet::on_update(AppState::Connect)
        .with_run_criteria(FixedTimestep::step(1.0))
        .with_system(systems::connect_to_server)
}
