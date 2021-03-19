use amethyst::{
    derive::SystemDesc,
    ecs::{System, SystemData, Read, ReadStorage, WriteStorage, WriteExpect, Join},
    shrev::{ReaderId, EventChannel},
};

use derive_new::new;
use std::collections::HashMap;
use westiny_common::{
    components::{NetworkId, Health},
    network::EntityHealth,
    resources::{AudioQueue, SoundId},
};


#[derive(SystemDesc, new)]
#[system_desc(name(HealthUpdateSystemDesc))]
pub struct HealthUpdateSystem {
    #[system_desc(event_channel_reader)]
    reader: ReaderId<EntityHealth>,
}

impl<'s> System<'s> for HealthUpdateSystem {
    type SystemData = (
        Read<'s, EventChannel<EntityHealth>>,
        ReadStorage<'s, NetworkId>,
        WriteStorage<'s, Health>,
        WriteExpect<'s, AudioQueue>,
        );

    fn run(&mut self, (health_updates_channel, network_ids, mut healths, mut audio): Self::SystemData) {
        let updates: HashMap<_, _> = health_updates_channel.read(&mut self.reader).map(|update| (update.network_id, update.health)).collect();

        for (net_id, health) in (&network_ids, &mut healths).join() {
            if let Some(&new_health) = updates.get(net_id)
            {
                if new_health.0 < health.0 {
                    audio.play(SoundId::Ouch, 1.0);
                }

                log::debug!("Health updated net_id={:?}, health=[{:?} -> {:?}]", net_id, health, new_health);
                *health = new_health;
            }
        }

    }
}
