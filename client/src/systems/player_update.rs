use amethyst::{
    derive::SystemDesc,
    ecs::{System, SystemData, Read, ReadStorage, WriteStorage, ReadExpect, WriteExpect, Join},
    shrev::{ReaderId, EventChannel},
};

use derive_new::new;
use westiny_common::components::{weapon::Weapon, Health, NetworkId};
use westiny_common::network::PlayerUpdate;
use westiny_common::resources::{AudioQueue, SoundId};
use crate::resources::PlayerNetworkId;

#[derive(new, SystemDesc)]
#[system_desc(name(PlayerUpdateSystemDesc))]
pub struct PlayerUpdateSystem {
    #[system_desc(event_channel_reader)]
    reader: ReaderId<PlayerUpdate>,
}

impl<'s> System<'s> for PlayerUpdateSystem {
    type SystemData = (
        Read<'s, EventChannel<PlayerUpdate>>,
        ReadStorage<'s, NetworkId>,
        WriteStorage<'s, Health>,
        WriteStorage<'s, Weapon>,
        ReadExpect<'s, PlayerNetworkId>,
        WriteExpect<'s, AudioQueue>,
        );

    fn run(&mut self, (player_updates_channel, net_ids, mut healths, mut weapons, player_net_id, mut audio): Self::SystemData) {
        let updates = player_updates_channel.read(&mut self.reader);
        if updates.len() == 0 { return; }

        let (health, weapon, _) = {
            if let Some(player) = (&mut healths, &mut weapons, &net_ids).join()
                .find(|(_, _, &net_id)| net_id == player_net_id.0) {
                player
            } else {
                log::error!("Player update received while player entity does not exist or does not have the required components");
                return;
            }
        };

        for player_update in updates {
            match player_update {
                PlayerUpdate::HealthUpdate(new_health) => {
                    if new_health.0 < health.0 {
                        audio.play(SoundId::Ouch, 1.0);
                    }
                    health.0 = new_health.0;
                    log::debug!("Health updated to {:?}", new_health);
                }
                PlayerUpdate::AmmoUpdate { ammo_in_magazine} => {
                    weapon.bullets_left_in_magazine = *ammo_in_magazine;
                    log::debug!("Ammo updated to {:?}", ammo_in_magazine);
                }
            }
        }
    }
}
