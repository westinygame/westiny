use crate::components::weapon::WeaponDetails;
use crate::resources::weapon::{GunResource, GunId};
use amethyst::ecs::Component;
use amethyst::ecs::DenseVecStorage;

/// This is the first approach of the inventory. For now it stores fix number of guns
#[derive(Component)]
#[storage(DenseVecStorage)]
pub struct Holster {
    pub guns: [WeaponDetails; 3]
}

impl Holster {
    pub fn new(gun_resource: &GunResource) -> Self {
        let guns = [
            gun_resource.get_gun(GunId::Revolver),
            gun_resource.get_gun(GunId::Shotgun),
            gun_resource.get_gun(GunId::Rifle)
        ];

        Holster { guns }
    }
}