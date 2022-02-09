use std::net::{SocketAddr, IpAddr};
use std::str::FromStr;
use serde::Deserialize;

//use crate::resources::GroundTile;
//use westiny_common::events::{WestinyEvent, WestinyEventReader};
use westiny_common::utilities::read_ron;
use crate::resources::ServerAddress;
use westiny_common::NetworkConfig;

use blaminar::simulation::{TransportResource, laminar::LaminarPlugin, NetworkSimulationEvent};

use bevy::prelude::*;

mod systems;
mod resources;
//mod entities;
mod states;
//mod bindings;
//mod components;

//#[cfg(test)]
//mod test_helpers;

fn main() {
    let application_root_dir = {
        let cargo_manifest_dir = std::env::var("CARGO_MANIFEST_DIR").unwrap();
        std::path::PathBuf::from(cargo_manifest_dir)
    };
    let common_resources_dir = application_root_dir.join("../resources");
    let resources_dir = application_root_dir.join("resources");

    let client_port: u16 = {
        let ron_path = resources_dir.join("client_network.ron");
        read_ron::<ClientPort>(&ron_path)
            .unwrap_or_else(|err| {
            let client_port: ClientPort = Default::default();
            log::warn!("Failed to read client network configuration file: {}, error: [{}] \
            Using default client port ({})",
                   ron_path.as_os_str().to_str().unwrap(),
                   err,
                   client_port.0);
            client_port
        }).0
    };
    let client_socket = SocketAddr::new(IpAddr::from_str("0.0.0.0").unwrap(), client_port);

    let laminar_config = {
        let ron_path = common_resources_dir.join("protocol.ron");
        read_ron::<NetworkConfig>(&ron_path)
            .map(|net_conf| net_conf.into())
            .expect(&format!("Failed to load Laminar protocol configuration file: {}", ron_path.as_os_str().to_str().unwrap()))
    };

    App::new()
        .add_plugins(DefaultPlugins)

        .insert_resource(get_server_address())
        .init_resource::<TransportResource>()

        .add_event::<NetworkSimulationEvent>()

        .add_plugin(LaminarPlugin::new(client_socket, laminar_config))

        .add_state(states::AppState::Connect)
        .add_system_set(states::connection::connect_state_systems())

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
        .map_err(|err| {
            anyhow::Error::from(err)
        })
        .and_then(|env| SocketAddr::from_str(&env)
            .map_err(|err|anyhow::Error::from(err)))
        .map(|addr| ServerAddress { address:addr });

    match address_result {
        Ok(addr) => {
            log::info!("Server address: {}", addr.address);
            addr
        }
        Err(err) => {
            let addr = ServerAddress::default();
            log::warn!("Server address has not been configured. Error: {}. Using default address: {}", err, addr.address);
            addr
        }
    }
}
