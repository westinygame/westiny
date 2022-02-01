use std::net::{IpAddr, SocketAddr};
use std::path::PathBuf;
use std::str::FromStr;

use westiny_common::resources::ServerAddress;
use westiny_common::utilities::read_ron;
use westiny_common::NetworkConfig;

use blaminar::simulation::laminar::LaminarPlugin;

use bevy::log::LogPlugin;
use bevy::prelude::*;

use crate::diagnostics::DiagnosticPlugins;
use crate::resources::{ClientNetworkEvent, ClientRegistry, NetworkCommand};

pub mod components;
pub mod diagnostics;
pub mod resources;
pub mod systems;

const WEAPONS_DIR: &'static str = "assets/weapons";

fn main() {
    let resources_dir = PathBuf::from("resources");
    let server_port: u16 = {
        let ron_path = resources_dir.join("server_network.ron");
        read_ron::<ServerAddress>(&ron_path)
            .map(|addr| addr.address.port())
            .unwrap_or_else(|err| {
                let srv_port = ServerAddress::default().address.port();
                log::warn!(
                    "Failed to read server network configuration file: {}, error: [{}] \
                Using default server port ({})",
                    ron_path.as_os_str().to_str().unwrap(),
                    err,
                    srv_port
                );
                srv_port
            })
    };

    let socket_address = SocketAddr::new(IpAddr::from_str("0.0.0.0").unwrap(), server_port);
    log::info!("Start listening on {}", socket_address);

    let laminar_config = {
        let ron_path = resources_dir.join("protocol.ron");
        read_ron::<NetworkConfig>(&ron_path)
            .map(|net_conf| net_conf.into())
            .expect(&format!(
                "Failed to load Laminar protocol configuration file: {}",
                ron_path.as_os_str().to_str().unwrap()
            ))
    };

    let weapons_path = resources_dir.join(WEAPONS_DIR);
    let gun_resource = resources::weapon::GunResource::load(&weapons_path)
        .expect(&format!("Unable to load weapons from directory: {:?}", std::fs::canonicalize(weapons_path).unwrap()));

    App::new()
        .insert_resource(ClientRegistry::new(64))
        .insert_resource(resources::Seed(0)) // Hard-coded seed for now
        .insert_resource(resources::NetworkIdSupplier::new())
        .insert_resource(gun_resource)
        .insert_resource(resources::ResourcesDir(resources_dir))
        .add_event::<ClientNetworkEvent>()
        .add_event::<NetworkCommand>()
        .add_event::<systems::SpawnPlayerEvent>()
        .add_event::<westiny_common::events::DamageEvent>()
        .add_event::<westiny_common::events::EntityDelete>()
        .add_plugins(MinimalPlugins)
        .add_plugin(LogPlugin)
        .add_plugins(DiagnosticPlugins)
        .add_plugin(LaminarPlugin::new(socket_address, laminar_config))
        // .add_plugin(RonAssetPlugin::<WeaponDetails>::new(&["gun"]))
        .add_startup_system(systems::build_map)
        .add_system(systems::read_network_messages.label("network_input"))
        .add_system(
            systems::introduce_new_clients
                .label("introduce_client")
                .after("network_input"),
        )
        .add_system(
            systems::spawn_player
                .label("spawn_player")
                .after("introduce_client"),
        )
        .add_system(
            systems::transform_commands
                .label("transform_commands")
                .after("network_input"),
        )
        .add_system(
            systems::apply_input
                .label("apply_input")
                .after("transform_commands"),
        )
        .add_system(systems::physics.label("physics").after("apply_input"))
        .add_system(
            systems::broadcast_entity_state
                .label("broadcast_entity_state")
                .after("introduce_client")
                .after("physics"),
        )
        .add_plugin(systems::CollisionPlugin)
        .add_system(
            systems::handle_damage
                .label("health")
                .after("projectile_collision"),
        )
        .add_system(systems::handle_death.label("death").after("health"))
        .add_system_set(systems::weapon_handler_system_set().label("weapon_handler"))
        .add_system(systems::lifespan_system.label("lifespan"))
        .add_system(systems::respawn_player.label("respawn").after("health"))
        .add_system_set(
            systems::entity_delete_system_set()
                .label("entity_delete_ss")
                .after("projectile_collision")
                .after("death")
                .after("respawn")
                .after("introduce_client")
                .after("lifespan"),
        )
        .run();
}
