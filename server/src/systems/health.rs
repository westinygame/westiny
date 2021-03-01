use amethyst::{
    derive::SystemDesc,
    ecs::{System, SystemData, Read, ReadStorage, WriteStorage},
    shrev::{ReaderId, EventChannel},
};

use derive_new::new;
use westiny_common::components::{NetworkId, Health};
use westiny_common::network::{EntityHealth, PacketType};
use crate::resources::{DamageEvent, ClientRegistry, StreamId, ClientID};
use amethyst::core::ecs::{ReadExpect, WriteExpect, Entity};
use crate::components::{Client, Eliminated};
use amethyst::network::simulation::{TransportResource, DeliveryRequirement, UrgencyRequirement};
use westiny_common::serialize;

use anyhow;
use amethyst::core::Time;

#[derive(SystemDesc, new)]
#[system_desc(name(HealthSystemDesc))]
pub struct HealthSystem {
    #[system_desc(event_channel_reader)]
    reader: ReaderId<DamageEvent>,
}

impl<'s> System<'s> for HealthSystem {
    type SystemData = (
        Read<'s, EventChannel<DamageEvent>>,
        WriteStorage<'s, Health>,
        ReadStorage<'s, NetworkId>,
        ReadStorage<'s, Client>,
        WriteStorage<'s, Eliminated>,
        ReadExpect<'s, ClientRegistry>,
        WriteExpect<'s, TransportResource>,
        ReadExpect<'s, Time>,
    );

    fn run(&mut self, data: Self::SystemData) {
        let (
            damage_event_channel,
            mut healths,
            net_ids,
            clients,
            mut eliminates,
            client_registry,
            mut transport,
            time,
        ) = data;

        for damage_event in damage_event_channel.read(&mut self.reader) {
            if let Some(health) = healths.get_mut(damage_event.target) {
                let health_drained = health.0 <= damage_event.damage.0;
                if health_drained {
                    health.0 = 0;
                    if let Err(err) = eliminates.insert(damage_event.target, Eliminated { elimination_time_sec: time.absolute_time_seconds() }) {
                        log::error!("Component 'Eliminated' could not be inserted to entity. error: {:?}", err);
                    }
                } else {
                    *health -= damage_event.damage;

                    if let Some(client) = clients.get(damage_event.target) {
                        log::debug!("Client [id: {:?}] took {} damage", client.id, damage_event.damage.0);
                        if let Err(err) = HealthSystem::notify_client(&net_ids, &client_registry, &mut transport, damage_event.target, health.clone(), &client.id) {
                            log::error!("Error while sending Health update to client: {}", err);
                        }
                    }
                }
            }
        }
    }
}

impl HealthSystem {
    fn notify_client(net_ids: &ReadStorage<'_, NetworkId>,
                     client_registry: &ClientRegistry,
                     transport: &mut TransportResource,
                     target_entity: Entity,
                     new_health: Health,
                     client: &ClientID) -> anyhow::Result<()> {
        let client_handle = {
            // could not convert with '?' even after .ok_or(...)
            let result = client_registry.find_client(*client);
            if result.is_none() {
                return Err(anyhow::anyhow!("Client [id: {:?}] not found in registry", client));
            }
            result.unwrap()
        };

        let &network_id = {
            let result = net_ids.get(target_entity);
            if result.is_none() {
                return Err(anyhow::anyhow!("Network id not found for client's player entity [client_id: {:?}]"));
            }
            result.unwrap()
        };

        let payload = serialize(&PacketType::EntityHealthUpdate(
            EntityHealth {
                network_id,
                health: new_health
            }
        )).map_err(|err| anyhow::Error::new(err))?;

        transport.send_with_requirements(
            client_handle.addr,
            &payload,
            DeliveryRequirement::ReliableSequenced(StreamId::HealthUpdate.into()),
            UrgencyRequirement::OnTick
        );

        Ok(())
    }
}
