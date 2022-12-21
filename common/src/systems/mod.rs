pub use collision::CollisionPlugin;
pub use lifespan::lifespan_system;
pub use physics::physics;

mod collision;
mod lifespan;
mod physics;

use crate::resources;
use bevy::prelude::{Commands, Res};

pub fn build_map(
    commands: Commands,
    seed: Res<resources::Seed>,
    res_dir: Res<resources::ResourcesDir>,
) {
    let res = resources::map::build_map(commands, *seed, &res_dir.common_resources.join("map"));
    match res {
        Ok(()) => bevy::log::info!("Map built"),
        Err(err) => bevy::log::error!("{}", err),
    };
}
