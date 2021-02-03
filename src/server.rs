use amethyst::utils::application_root_dir;
use amethyst::{GameDataBuilder, CoreApplication};
use amethyst::network::simulation::laminar::{LaminarNetworkBundle, LaminarSocket, LaminarConfig};
use crate::{
    resources::ServerAddress,
    utilities::*,
};
use std::net::{IpAddr, SocketAddr};
use std::str::FromStr;
use std::time::Duration;

mod systems;
mod entities;
mod components;
mod resources;
mod states;
mod events;
mod network;
mod utilities;

#[cfg(test)]
mod test_helpers;

fn main() -> amethyst::Result<()> {
    amethyst::start_logger(Default::default());

    let app_root = application_root_dir()?;
    let resources_dir = app_root.join("resources");

    let server_port: u16 = {
        let ron_path = resources_dir.join("server_network.ron");
        read_ron(&ron_path).unwrap_or_else(|_| {
            let srv_port = ServerAddress::default().address.port();
            log::warn!("Failed to read server network configuration file: {}, \
            Using default server port ({})",
                       ron_path.as_os_str().to_str().unwrap(),
                       srv_port);
            srv_port
        })
    };
    let socket_address = SocketAddr::new(IpAddr::from_str("127.0.0.1").unwrap(), server_port);

    let laminar_config = {
        let mut conf = LaminarConfig::default();
        // send heartbeat in every 3 seconds
        conf.heartbeat_interval = Some(Duration::from_secs(3));
        conf
    };
    let socket = LaminarSocket::bind_with_config(socket_address, laminar_config)?;

    let game_data = GameDataBuilder::default()
        .with_bundle(LaminarNetworkBundle::new(Some(socket)))?
        .with_system_desc(systems::server_network::ServerNetworkSystemDesc::default(), "game_network", &["network_recv"]);

    let mut game =
        CoreApplication::<_, events::WestinyEvent, events::WestinyEventReader>::build(
            resources_dir,
            states::server_states::ServerState::default(),
        )?.build(game_data)?;

    log::info!("Starting server");
    game.run();
    Ok(())
}