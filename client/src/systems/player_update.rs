use bevy::prelude::*;
use westiny_common::components::{Health, Player};
use westiny_common::network::{PlayerUpdate, PlayerNotification};
use crate::components::WeaponInfo;
use westiny_common::resources::{AudioQueue, SoundId};

pub fn update_player(
    mut update_events: EventReader<PlayerUpdate>,
    mut player_state: Query<(&mut Health, &mut WeaponInfo), With<Player>>
) {
    let (mut health, mut weapon_info) = player_state.single_mut();

    for player_update in update_events.iter() {
        match player_update {
            PlayerUpdate::HealthUpdate(new_health) => {
                if new_health.0 < health.0 {
        //            audio.play(SoundId::Ouch, 1.0);
                }
                health.0 = new_health.0;
                log::debug!("Health updated to {:?}", new_health);
            }
            PlayerUpdate::AmmoUpdate { ammo_in_magazine } => {
                if ammo_in_magazine > &weapon_info.bullets_in_magazine {
         //           audio.play(SoundId::WeaponReady, 1.0);
                }
                weapon_info.bullets_in_magazine = *ammo_in_magazine;
                log::debug!("Ammo updated to {:?}", ammo_in_magazine);
            }
            PlayerUpdate::WeaponSwitch {name, magazine_size, ammo_in_magazine} => {
                weapon_info.name = name.clone();
                weapon_info.magazine_size = *magazine_size;
                weapon_info.bullets_in_magazine = *ammo_in_magazine;
                log::debug!("Weapon updated");

                //notification.single_write(PlayerNotification { message: format!("Weapon: {}.", name) })
            }
        }
    }
}
