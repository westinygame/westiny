use amethyst::core::ecs::World;
use crate::components::weapon::WeaponDetails;
use std::path::Path;
use crate::utilities::read_ron;

pub struct GunResource {
    weapons: [WeaponDetails; 1],
}

#[derive(Copy, Clone, PartialEq, Eq)]
#[repr(usize)]
pub enum GunId {
    Revolver = 0,
}

const WEAPON_ASSET_DIR: &'static str = "assets/weapons/";

impl GunResource {
    pub fn initialize<P: AsRef<Path>>(world: &mut World, resources_path: P) -> anyhow::Result<()>{
        let path = resources_path.as_ref().join(WEAPON_ASSET_DIR);

        let revolver: WeaponDetails = read_ron(&path.join("revolver.ron"))?;
        // other weapons here

        world.insert(GunResource { weapons: [revolver]});
        Ok(())
    }

    pub fn get_gun(&self, id: GunId) -> WeaponDetails {
        self.weapons[id as usize].clone()
        // panics if there's no WeaponDetails at the given GunId. There should be
    }
}