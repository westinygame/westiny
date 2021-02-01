use amethyst::utils::application_root_dir;
use amethyst::{GameDataBuilder, CoreApplication};
use amethyst::core::TransformBundle;
use amethyst::renderer::{RenderingBundle, RenderToWindow, RenderFlat2D, types::DefaultBackend};
use amethyst::tiles::{RenderTiles2D, MortonEncoder};
use amethyst::network::simulation::laminar::{LaminarSocket, LaminarNetworkBundle, LaminarConfig};
use std::time::Duration;

mod systems;
mod entities;
mod components;
mod resources;
mod states;
mod events;
mod network;

#[cfg(test)]
mod test_helpers;

fn main() -> amethyst::Result<()> {
    amethyst::start_logger(Default::default());

    let app_root = application_root_dir()?;
    let resources_dir = app_root.join("resources");
    let display_config = resources_dir.join("display_config.ron");

    let laminar_config = {
        let mut conf = LaminarConfig::default();
        // send heartbeat in every 3 seconds
        conf.heartbeat_interval = Some(Duration::from_secs(3));
        conf
    };
    let socket = LaminarSocket::bind_with_config("127.0.0.1:1234", laminar_config).unwrap();

    let game_data = GameDataBuilder::default()
        .with_bundle(TransformBundle::new())?
        .with_bundle(RenderingBundle::<DefaultBackend>::new()
            .with_plugin(
                RenderToWindow::from_config_path(display_config)?
                    .with_clear([0.0, 0.0, 0.0, 1.0])
            )
            .with_plugin(RenderFlat2D::default())
            .with_plugin(RenderTiles2D::<resources::GroundTile, MortonEncoder>::default()))?
        .with_bundle(LaminarNetworkBundle::new(Some(socket)))?;

    let mut game =
        CoreApplication::<_, events::WestinyEvent, events::WestinyEventReader>::build(
            resources_dir,
            states::connection::ConnectState::default(),
        )?.build(game_data)?;

    log::info!("Starting client");
    game.run();
    Ok(())
}