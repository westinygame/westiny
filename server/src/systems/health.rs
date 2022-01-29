use crate::resources::{ClientRegistry, StreamId, ClientID};
use crate::components::{Client, Eliminated, Health};
use westiny_common::events::DamageEvent;
use westiny_common::network::{PacketType, PlayerUpdate};
use westiny_common::serialization::serialize;
use blaminar::simulation::{TransportResource, UrgencyRequirement, DeliveryRequirement};
use bevy::prelude::{EventReader, Commands, Time, Query, Res, ResMut};
use bevy::ecs::system::Insert;

use anyhow;

pub fn handle_damage(mut commands: Commands,
                     time: Res<Time>,
                     mut damage_ec: EventReader<DamageEvent>,
                     mut healths: Query<(&mut Health, Option<&Client>)>,
                     client_registry: Res<ClientRegistry>,
                     mut transport: ResMut<TransportResource>) {
    for damage_event in damage_ec.iter() {
        if let Ok((mut health, maybe_client)) = healths.get_mut(damage_event.target) {
            let health_drained = health.0 <= damage_event.damage.0;
            if health_drained {
                health.0 = 0;
                commands.add(Insert {
                    entity: damage_event.target,
                    component: Eliminated {elimination_time_sec: time.seconds_since_startup()}
                });
            } else {
                *health -= damage_event.damage;
            }

            if let Some(client) = maybe_client {
                if let Err(err) = notify_client(&client_registry, &mut transport, health.clone(), &client.id) {
                    log::error!("Error while sending Health update to client: {}", err);
                }
            }
        }
    }
}

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
