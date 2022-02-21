use crate::components::weapon::WeaponDetails;
use crate::utilities::read_ron;

pub struct GunResource {
    weapons: [WeaponDetails; 3],
}

#[derive(Copy, Clone, PartialEq, Eq)]
#[repr(usize)]
pub enum GunId {
    Revolver = 0,
    Shotgun,
    Rifle,
}

impl GunResource {
    pub fn load(weapons_dir: &std::path::Path) -> anyhow::Result<Self> {
        let revolver = read_ron::<WeaponDetails>(&weapons_dir.join("revolver.ron"))?;
        let shotgun = read_ron::<WeaponDetails>(&weapons_dir.join("shotgun.ron"))?;
        let rifle = read_ron::<WeaponDetails>(&weapons_dir.join("rifle.ron"))?;

        Ok(Self {
            weapons: [revolver, shotgun, rifle],
        })
    }

    pub fn get_gun(&self, id: GunId) -> WeaponDetails {
        self.weapons[id as usize].clone()
        // panics if there's no WeaponDetails at the given GunId. There should be
    }
}
