use serde::Deserialize;
use std::net::{IpAddr, SocketAddr};
use std::str::FromStr;

//use crate::resources::GroundTile;
//use westiny_common::events::{WestinyEvent, WestinyEventReader};
use crate::resources::ServerAddress;
use westiny_common::{
    network::{
        EntityState, NetworkEntityDelete, PlayerDeath, PlayerNotification, PlayerUpdate, ShotEvent,
    },
    utilities::read_ron,
    NetworkConfig,
};

use blaminar::prelude::{LaminarLabel, LaminarPlugin, NetworkSimulationEvent, TransportResource};

use bevy::prelude::*;

mod entities;
mod resources;
mod states;
mod systems;
//mod bindings;
mod components;

//#[cfg(test)]
//mod test_helpers;

fn main() {
    let application_root_dir = {
        let cargo_manifest_dir = std::env::var("CARGO_MANIFEST_DIR").unwrap();
        std::path::PathBuf::from(cargo_manifest_dir)
    };
    let common_resources_dir = application_root_dir.join("../resources");
    let resources_dir = application_root_dir.join("assets");

    let client_port: u16 = {
        let ron_path = resources_dir.join("client_network.ron");
        read_ron::<ClientPort>(&ron_path)
            .unwrap_or_else(|err| {
                let client_port: ClientPort = Default::default();
                log::warn!(
                    "Failed to read client network configuration file: {}, error: [{}] \
            Using default client port ({})",
                    ron_path.as_os_str().to_str().unwrap(),
                    err,
                    client_port.0
                );
                client_port
            })
            .0
    };
    let client_socket = SocketAddr::new(IpAddr::from_str("0.0.0.0").unwrap(), client_port);

    let laminar_config = {
        let ron_path = common_resources_dir.join("protocol.ron");
        read_ron::<NetworkConfig>(&ron_path)
            .map(|net_conf| net_conf.into())
            .expect(&format!(
                "Failed to load Laminar protocol configuration file: {}",
                ron_path.as_os_str().to_str().unwrap()
            ))
    };

    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugin(LaminarPlugin::new(client_socket, laminar_config))
        .insert_resource(get_server_address())
        .insert_resource(resources::Seed(10))
        .insert_resource(resources::ResourcesDir {
            common_resources: common_resources_dir,
            crate_resources: resources_dir,
        })
        .init_resource::<TransportResource>()
        .init_resource::<resources::SpriteResource>()
        .init_resource::<resources::PlayerNetworkId>()
        .add_event::<NetworkSimulationEvent>()
        .add_event::<Vec<EntityState>>()
        .add_event::<PlayerUpdate>()
        .add_event::<PlayerDeath>()
        .add_event::<PlayerNotification>()
        .add_event::<NetworkEntityDelete>()
        .add_event::<ShotEvent>()
        .add_state(states::AppState::Connect)
        .add_startup_system(resources::initialize_sprite_resource)
        .add_system_to_stage(CoreStage::PostUpdate, systems::add_sprite_to_new_sprite_id)
        .add_system_set(
            SystemSet::on_update(states::AppState::Connect)
                .after(LaminarLabel)
                .with_system(systems::send_connection_request)
                .with_system(systems::receive_connection_response),
        )
        .add_system_set(
            SystemSet::on_exit(states::AppState::Connect)
                .with_system(|| log::debug!("Exitting connect state")),
        )
        .add_system_set(states::play::setup_system_set())
        .add_system_set(states::play::system_set())
        .run();
}

/*
fn main() -> amethyst::Result<()> {
    let display_config = resources_dir.join("display_config.ron");

    let key_bindings = resources_dir.join("input.ron");
    let input_bundle = InputBundle::<bindings::MovementBindingTypes>::new().with_bindings_from_file(key_bindings)?;

    let game_data = GameDataBuilder::default()
        .with_bundle(TransformBundle::new())?
        .with_bundle(input_bundle)?
        .with_bundle(UiBundle::<bindings::MovementBindingTypes>::new())?
        .with_bundle(RenderingBundle::<DefaultBackend>::new()
            .with_plugin(
                RenderToWindow::from_config_path(display_config)?
                    .with_clear([0.0, 0.0, 0.0, 1.0])
            )
            .with_plugin(RenderFlat2D::default())
            .with_plugin(RenderTiles2D::<GroundTile, MortonEncoder>::default())
            .with_plugin(RenderUi::default())
            )?
        .with_bundle(LaminarNetworkBundle::new(Some(socket)))?
        .with_bundle(AudioBundle::default())?
        ;

    let mut game =
        CoreApplication::<_, WestinyEvent, WestinyEventReader>::build(
            &resources_dir,
            states::connection::ConnectState::new(&common_resources_dir),
        )?.build(game_data)?;

    log::info!("Starting client");
    game.run();
    Ok(())
}
*/

const DEFAULT_CLIENT_PORT: u16 = 4557;

#[derive(Deserialize)]
pub struct ClientPort(pub u16);
impl Default for ClientPort {
    fn default() -> Self {
        ClientPort(DEFAULT_CLIENT_PORT)
    }
}

fn get_server_address() -> ServerAddress {
    let address_result = std::env::var("WESTINY_SERVER_ADDRESS")
        .map_err(|err| anyhow::Error::from(err))
        .and_then(|env| SocketAddr::from_str(&env).map_err(|err| anyhow::Error::from(err)))
        .map(|addr| ServerAddress { address: addr });

    match address_result {
        Ok(addr) => {
            log::info!("Server address: {}", addr.address);
            addr
        }
        Err(err) => {
            let addr = ServerAddress::default();
            log::warn!(
                "Server address has not been configured. Error: {}. Using default address: {}",
                err,
                addr.address
            );
            addr
        }
    }
}
