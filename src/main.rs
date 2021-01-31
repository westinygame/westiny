use amethyst::core::transform::TransformBundle;
use amethyst::input::{InputBundle, StringBindings};
use amethyst::prelude::*;
use amethyst::renderer::RenderingBundle;
use amethyst::renderer::plugins::{RenderFlat2D, RenderToWindow};
use amethyst::renderer::types::DefaultBackend;
use amethyst::utils::application_root_dir;
use amethyst::tiles::{RenderTiles2D, MortonEncoder};

use log::info;

mod state;
mod systems;
mod entities;
mod components;
mod resources;
mod states;
mod events;
mod network;

#[cfg(test)]
mod test_helpers;

/// Desert sand color
const BACKGROUND_COLOR: [f32; 4] = [0.75, 0.65, 0.5, 1.0];

fn main() -> amethyst::Result<()> {
    amethyst::start_logger(Default::default());

    let app_root = application_root_dir()?;
    let resources_dir = app_root.join("resources");
    let display_config = resources_dir.join("display_config.ron");

    let game_data = GameDataBuilder::default()
        .with_bundle(TransformBundle::new())?
        .with_bundle(RenderingBundle::<DefaultBackend>::new()
            .with_plugin(
                RenderToWindow::from_config_path(display_config)?
                    .with_clear(BACKGROUND_COLOR)
            )
            .with_plugin(RenderFlat2D::default())
            .with_plugin(RenderTiles2D::<resources::GroundTile, MortonEncoder>::default())
        )?;

    let mut game = CoreApplication::<_, events::WestinyEvent, events::WestinyEventReader>::build(
        resources_dir,
        states::client_states::ConnectState::default(),
    )?.build(game_data)?;

    info!("Starting...");
    game.run();

    Ok(())
}
