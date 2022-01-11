use std::path::PathBuf;
use std::net::{SocketAddr, IpAddr};
use std::str::FromStr;

use westiny_common::resources::ServerAddress;
use westiny_common::utilities::read_ron;
use westiny_common::NetworkConfig;

use blaminar::simulation::laminar::LaminarPlugin;

use bevy::prelude::*;
use bevy::log::LogPlugin;

use crate::resources::{ClientRegistry, ClientNetworkEvent, NetworkCommand};
use crate::diagnostics::DiagnosticPlugins;

pub mod resources;
pub mod systems;
pub mod components;
pub mod server_state;
pub mod diagnostics;

fn main() {
    let resources_dir = PathBuf::from("resources");
    let server_port: u16 = {
        let ron_path = resources_dir.join("server_network.ron");
        read_ron::<ServerAddress>(&ron_path)
            .map(|addr| addr.address.port())
            .unwrap_or_else(|err|{
                let srv_port = ServerAddress::default().address.port();
                log::warn!("Failed to read server network configuration file: {}, error: [{}] \
                Using default server port ({})",
                           ron_path.as_os_str().to_str().unwrap(),
                           err,
                           srv_port);
                srv_port
            })
    };

    let socket_address = SocketAddr::new(IpAddr::from_str("0.0.0.0").unwrap(), server_port);
    log::info!("Start listening on {}", socket_address);

    let laminar_config = {
    let ron_path = resources_dir.join("protocol.ron");
    read_ron::<NetworkConfig>(&ron_path)
        .map(|net_conf| net_conf.into())
        .expect(&format!("Failed to load Laminar protocol configuration file: {}", ron_path.as_os_str().to_str().unwrap()))
    };


    App::build()
        .insert_resource(ClientRegistry::new(64))
        .insert_resource(resources::Seed(0)) // Hard-coded seed for now
        .insert_resource(resources::NetworkIdSupplier::new())
        .insert_resource(resources::ResourcesDir(resources_dir))

        .add_plugins(MinimalPlugins)
        .add_plugin(LogPlugin)
        .add_plugins(DiagnosticPlugins)
        .add_plugin(LaminarPlugin::new(socket_address, laminar_config))
        // .add_plugin(RonAssetPlugin::<WeaponDetails>::new(&["gun"]))

        .add_startup_system(systems::build_map.system())

        .add_event::<ClientNetworkEvent>()
        .add_event::<NetworkCommand>()
        .add_event::<systems::SpawnPlayerEvent>()
        .add_event::<westiny_common::events::DamageEvent>()
        .add_event::<westiny_common::events::EntityDelete>()

        .add_system(systems::read_network_messages.system()
                    .label("network_input"))

        .add_system(systems::introduce_new_clients.system()
                    .label("introduce_client")
                    .after("network_input"))
        .add_system(systems::spawn_player.system()
                    .label("spawn_player")
                    .after("introduce_client"))

        .add_system(systems::transform_commands.system()
                    .label("transform_commands")
                    .after("network_input"))
        .add_system(systems::apply_input.system()
                    .label("apply_input")
                    .after("transform_commands"))
        .add_system(systems::physics.system()
                    .label("physics")
                    .after("apply_input"))

        .add_system(systems::broadcast_entity_state.system()
                    .label("broadcast_entity_state")
                    .after("introduce_client")
                    .after("physics"))

        .add_plugin(systems::CollisionPlugin)

        .add_system_set(systems::weapon_handler_system_set())
        .run();
}

// fn main() -> amethyst::Result<()> {
//     amethyst::start_logger(Default::default());
//
//     let app_root = application_root_dir()?;
//     let resources_dir = app_root.join(RESOURCES_RELATIVE_PATH);
//
//     let server_port: u16 = {
//         let ron_path = resources_dir.join("server_network.ron");
//         read_ron::<ServerAddress>(&ron_path)
//             .map(|addr| addr.address.port())
//             .unwrap_or_else(|err| {
//                 let srv_port = ServerAddress::default().address.port();
//                 log::warn!("Failed to read server network configuration file: {}, error: [{}] \
//                 Using default server port ({})",
//                            ron_path.as_os_str().to_str().unwrap(),
//                            err,
//                            srv_port);
//                 srv_port
//             })
//     };
//     let socket_address = SocketAddr::new(IpAddr::from_str("0.0.0.0").unwrap(), server_port);
//     log::info!("Start listening on {}", socket_address);
//
//     let laminar_config = {
//         let ron_path = resources_dir.join("protocol.ron");
//         read_ron::<NetworkConfig>(&ron_path)
//             .map(|net_conf| net_conf.into())
//             .expect(&format!("Failed to load Laminar protocol configuration file: {}", ron_path.as_os_str().to_str().unwrap()))
//     };
//
//     let socket = LaminarSocket::bind_with_config(socket_address, laminar_config)?;
//
//     let game_data = GameDataBuilder::default()
//         .with_bundle(LaminarNetworkBundle::new(Some(socket)))?
//         .with(systems::EntityStateBroadcasterSystem, "entity_state_broadcaster", &[])
//         .with_system_desc(systems::NetworkMessageReceiverSystemDesc::default(), "msg_receiver", &[])
//         .with_system_desc(systems::ClientIntroductionSystemDesc::default(), "client_intro", &["msg_receiver"])
//         .with_system_desc(systems::CommandTransformerSystemDesc::default(), "command_transformer", &["msg_receiver"])
//         .with(systems::PlayerMovementSystem, "player_movement", &["command_transformer"])
//         .with(systems::PhysicsSystem, "physics", &["player_movement"])
//         .with_bundle(CollisionBundle)?
//         .with(systems::LifespanSystem, "timing", &["collision"])
//         .with(systems::ShooterSystem, "shooter", &["command_transformer"])
//         .with_system_desc(systems::HealthSystemDesc::default(), "health", &["projectile_collision_handler"])
//         .with(systems::DeathSystem, "death", &["health"])
//         .with(systems::RespawnSystem, "respawn", &["death"])
//         .with_system_desc(systems::SpawnSystemDesc::default(), "spawn", &["client_intro", "respawn"])
//         .with_system_desc(systems::EntityDeleteBroadcasterSystemDesc::default(), "delete_broadcaster", &["collision_handler"])
//         ;
//
//     let frame_limit = 60;
//
//     let mut game =
//         CoreApplication::<_, WestinyEvent, WestinyEventReader>::build(
//             resources_dir.clone(),
//             server_state::ServerState::new(resources_dir),
//         )?
//         .with_frame_limit(
//             FrameRateLimitStrategy::SleepAndYield(Duration::from_millis(2)),
//             frame_limit
//         )
//         .build(game_data)?;
//
//     log::info!("Starting server");
//     game.run();
//     Ok(())
// }
