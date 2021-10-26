use crate::components::weapon::WeaponDetails;
use bevy::prelude::Commands;
use bevy::asset::{AssetServer, Handle};

pub struct GunResource {
    weapons: [Handle<WeaponDetails>; 3],
}

#[derive(Copy, Clone, PartialEq, Eq)]
#[repr(usize)]
pub enum GunId {
    Revolver = 0,
    Shotgun,
    Rifle,
}

const WEAPON_ASSET_DIR: &'static str = "assets/weapons/";

impl GunResource {
    pub fn setup_gun_resource(commands: &mut Commands, asset_server: &mut AssetServer) {
        let revolver = asset_server.load("revolver.gun");
        let shotgun = asset_server.load("shotgun.gun");
        let rifle = asset_server.load("rifle.gun");

        commands.insert_resource(GunResource { weapons: [revolver, shotgun, rifle] });
    }

    pub fn get_gun(&self, id: GunId) -> Handle<WeaponDetails> {
        self.weapons[id as usize].clone()
        // panics if there's no WeaponDetails at the given GunId. There should be
    }
}