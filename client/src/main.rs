use amethyst::audio::AudioBundle;
use amethyst::utils::application_root_dir;
use amethyst::{GameDataBuilder, CoreApplication};
use amethyst::core::TransformBundle;
use amethyst::renderer::{RenderingBundle, RenderToWindow, RenderFlat2D, types::DefaultBackend};
use amethyst::ui::{RenderUi, UiBundle};
use amethyst::tiles::{RenderTiles2D, MortonEncoder};
use amethyst::network::simulation::laminar::{LaminarSocket, LaminarNetworkBundle, LaminarConfig};
use std::time::Duration;
use std::net::{SocketAddr, IpAddr};
use std::str::FromStr;
use serde::Deserialize;
use amethyst::input::InputBundle;

use crate::resources::GroundTile;
use westiny_common::events::{WestinyEvent, WestinyEventReader};
use westiny_common::utilities::read_ron;

mod systems;
mod resources;
mod entities;
mod states;
mod bindings;

#[cfg(test)]
mod test_helpers;

fn main() -> amethyst::Result<()> {
    amethyst::start_logger(Default::default());

    let app_root = application_root_dir()?;
    let common_resources_dir = app_root.join("../resources");
    let resources_dir = app_root.join("assets");
    let display_config = resources_dir.join("display_config.ron");

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
    let client_socket = SocketAddr::new(IpAddr::from_str("0.0.0.0")?, client_port);

    let laminar_config = {
        let mut conf = LaminarConfig::default();
        // send heartbeat in every 3 seconds
        conf.heartbeat_interval = Some(Duration::from_secs(3));
        conf
    };
    let socket = LaminarSocket::bind_with_config(client_socket, laminar_config)?;
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

const DEFAULT_CLIENT_PORT: u16 = 4557;

#[derive(Deserialize)]
pub struct ClientPort(pub u16);
impl Default for ClientPort {
    fn default() -> Self {
        ClientPort(DEFAULT_CLIENT_PORT)
    }
}
