use amethyst::{
    derive::SystemDesc,
    ecs::{System, SystemData, Read, ReadStorage, WriteStorage, Join},
    shrev::{ReaderId, EventChannel},
};

use derive_new::new;
use std::collections::HashMap;
use crate::components::{NetworkId, Health};
use crate::network::EntityHealth;


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
        );

    fn run(&mut self, (health_updates_channel, network_ids, mut healths): Self::SystemData) {
        let updates: HashMap<_, _> = health_updates_channel.read(&mut self.reader).map(|update| (update.network_id, update.health)).collect();

        for (net_id, Health(health)) in (&network_ids, &mut healths).join() {
            if let Some(new_health) = updates.get(net_id)
            {
                log::debug!("Health updated net_id={:?}, health=[{:?} -> {:?}]", net_id, health, new_health);
                *health = *new_health;
            }
        }
    }
}
