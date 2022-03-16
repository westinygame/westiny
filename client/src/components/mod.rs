pub use weapon_info::WeaponInfo;
pub use westiny_common::components::*;

mod weapon_info;

pub mod hud {
    use bevy::prelude::Component;

    #[derive(Component)]
    pub struct Health;
}
