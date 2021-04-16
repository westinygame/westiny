use amethyst::{
    derive::SystemDesc,
    ecs::{System, SystemData, Read, ReadStorage, WriteStorage},
    shrev::{ReaderId, EventChannel},
};

use derive_new::new;
use westiny_common::components::Health;
use westiny_common::network::PacketType;
use crate::resources::{ClientRegistry, StreamId, ClientID};
use amethyst::core::ecs::{ReadExpect, WriteExpect};
use crate::components::{Client, Eliminated};
use amethyst::network::simulation::{TransportResource, DeliveryRequirement, UrgencyRequirement};
use westiny_common::serialize;

use anyhow;
use amethyst::core::Time;
use westiny_common::events::DamageEvent;
use westiny_common::network::PlayerUpdate;

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
                }
                if let Some(client) = clients.get(damage_event.target) {
                    log::debug!("Client [id: {:?}] took {} damage", client.id, damage_event.damage.0);
                    if let Err(err) = HealthSystem::notify_client(&client_registry, &mut transport, health.clone(), &client.id) {
                        log::error!("Error while sending Health update to client: {}", err);
                    }
                }
            }
        }
    }
}

impl HealthSystem {
    fn notify_client(client_registry: &ClientRegistry,
                     transport: &mut TransportResource,
                     new_health: Health,
                     client: &ClientID) -> anyhow::Result<()> {
        let client_handle = {
            client_registry.find_client(*client)
                .ok_or(anyhow::anyhow!("Client [id: {:?}] not found in registry", client))?
        };

        let payload = serialize(&PacketType::PlayerUpdate(PlayerUpdate::HealthUpdate(new_health)))
            .map_err(|err| anyhow::Error::new(err))?;

        transport.send_with_requirements(
            client_handle.addr,
            &payload,
            DeliveryRequirement::ReliableSequenced(StreamId::HealthUpdate.into()),
            UrgencyRequirement::OnTick
        );

        Ok(())
    }
}
