use amethyst::ecs::{Component, VecStorage};

#[derive(Debug, Clone, Component)]
#[storage(VecStorage)]
pub struct WeaponInfo {
    pub magazine_size: u32,
    pub bullets_in_magazine: u32,
    pub name: String
}

impl Default for WeaponInfo {
    fn default() -> Self {
        WeaponInfo {
            magazine_size: 0,
            bullets_in_magazine: 0,
            name: "None".to_string(),
        }
    }
}