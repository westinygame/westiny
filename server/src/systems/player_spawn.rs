
use amethyst::{
    derive::SystemDesc,
    ecs::{System, SystemData, Read, WriteExpect, ReadExpect, Entities, WriteStorage, Join},
    shrev::{ReaderId, EventChannel},
};

use derive_new::new;
use anyhow::Result;

use westiny_common::network::{PacketType, ClientInitialData};
use crate::resources::{ClientRegistry, ClientNetworkEvent, NetworkIdSupplier, ClientID};
use amethyst::network::simulation::{TransportResource, DeliveryRequirement, UrgencyRequirement};
use amethyst::core::Transform;
use crate::components;


#[derive(SystemDesc, new)]
#[system_desc(name(PlayerSpawnSystemDesc))]
pub struct PlayerSpawnSystem {
    #[system_desc(event_channel_reader)]
    reader: ReaderId<ClientNetworkEvent>,
}

impl<'s> System<'s> for PlayerSpawnSystem {
    type SystemData = (
        Read<'s, EventChannel<ClientNetworkEvent>>,
        Entities<'s>,
        WriteExpect<'s, TransportResource>,
        ReadExpect<'s, ClientRegistry>,
        WriteExpect<'s, NetworkIdSupplier>,

        // storages required for player creation
        WriteStorage<'s, components::Client>,
        WriteStorage<'s, components::NetworkId>,
        WriteStorage<'s, components::Player>,
        WriteStorage<'s, Transform>,
        WriteStorage<'s, components::Input>,
        WriteStorage<'s, components::Velocity>,
        WriteStorage<'s, components::weapon::Weapon>,
        WriteStorage<'s, components::BoundingCircle>,
    );

    fn run(
        &mut self,
        (
            client_net_ec,
            entities,
            mut net,
            client_registry,
            mut net_id_supplier,
            mut client,
            mut net_id,
            mut player,
            mut transform,
            mut input,
            mut velocity,
            mut weapon,
            mut bounding_circle
        ): Self::SystemData) {

        for client_network_event in client_net_ec.read(&mut self.reader) {
            match client_network_event {
                ClientNetworkEvent::ClientConnected(client_id) => {
                    let client_handle = client_registry.find_client(*client_id)
                        .expect(&format!("Client [client_id: {:?}] not found in registry", client_id));

                    let entity_network_id = Self::spawn_player(
                        &entities,
                        client_id,
                        &mut net_id_supplier,
                        &mut client,
                        &mut net_id,
                        &mut player,
                        &mut transform,
                        &mut input,
                        &mut velocity,
                        &mut weapon,
                        &mut bounding_circle
                    );
                    log::debug!("Player created for {}", client_handle.player_name);

                    // Send response to client
                    let connection_response = PacketType::ConnectionResponse(
                        Ok(ClientInitialData{
                            player_network_id: *entity_network_id
                        })
                    );
                    net.send_with_requirements(
                        client_handle.addr,
                        &bincode::serialize(&connection_response).unwrap(),
                        DeliveryRequirement::Reliable,
                        UrgencyRequirement::OnTick
                    )

                }
                ClientNetworkEvent::ClientDisconnected(client_id) => {
                    match Self::despawn_player(&entities, &mut client, client_id) {
                        Ok(()) => log::debug!("Disconnecting client's player entity [client_id: {:?}], has been removed", client_id),
                        Err(err) => log::error!("{}", err)
                    }
                }
            }
        }
    }
}

impl PlayerSpawnSystem {
    fn spawn_player(
        entities: &Entities<'_>,
        client_id: &ClientID,
        net_id_supplier: &mut NetworkIdSupplier,
        client_storage: &mut WriteStorage<'_, components::Client>,
        net_id_storage: &mut WriteStorage<'_, components::NetworkId>,
        player_storage: &mut WriteStorage<'_, components::Player>,
        transform_storage: &mut WriteStorage<'_, Transform>,
        input_storage: &mut WriteStorage<'_, components::Input>,
        velocity_storage: &mut WriteStorage<'_, components::Velocity>,
        weapon_storage: &mut WriteStorage<'_, components::weapon::Weapon>,
        bounding_circle_storage: &mut WriteStorage<'_, components::BoundingCircle>,
    ) -> components::NetworkId {
        use components::weapon;

        let transform = {
            let mut t = Transform::default();
            t.set_translation_xyz(0.0, 0.0, 0.0);
            t
        };

        // TODO define these values in RON resource files. PREFAB?
        let revolver = weapon::WeaponDetails {
            damage: 5.0,
            distance: 120.0,
            fire_rate: 7.2,
            magazine_size: 6,
            reload_time: 1.0,
            spread: 2.0,
            shot: weapon::Shot::Single,
            bullet_speed: 200.0
        };

        let client = components::Client::new(*client_id);
        let network_id = net_id_supplier.next();

        entities.build_entity()
            .with(client, client_storage)
            .with(network_id, net_id_storage)
            .with(components::Player, player_storage)
            .with(transform, transform_storage)
            .with(components::Input::default(), input_storage)
            .with(components::Velocity::default(), velocity_storage)
            .with(components::weapon::Weapon::new(revolver), weapon_storage)
            .with(components::BoundingCircle{radius: 8.0}, bounding_circle_storage)
            .build();

        network_id
    }

    fn despawn_player(
        entities: &Entities<'_>,
        client_storage: &mut WriteStorage<'_, components::Client>,
        client_id: &ClientID,
    ) -> Result<()>{
        for (entity, client) in (&*entities, client_storage).join() {
            if client.id() == client_id {
                return match entities.delete(entity) {
                    Ok(_) =>  Ok(()),
                    Err(err) => Err(anyhow::anyhow!(
                        "Disconnecting client's player entity [client_id: {:?}] could not be removed. {}",
                        client_id,
                        err
                    ))
                }
            }
        }

        Err(anyhow::anyhow!(
        "Disconnecting client's player entity [client_id: {:?}] not found thus could not be removed",
        client_id))
    }
}