use amethyst::utils::application_root_dir;
use amethyst::{GameDataBuilder, CoreApplication};
use amethyst::network::simulation::laminar::{LaminarNetworkBundle, LaminarSocket};

mod state;
mod systems;
mod entities;
mod components;
mod resources;
mod states;
mod events;

#[cfg(test)]
mod test_helpers;

fn main() -> amethyst::Result<()> {
    amethyst::start_logger(Default::default());

    let app_root = application_root_dir()?;
    let resources_dir = app_root.join("resources");

    let socket = LaminarSocket::bind("127.0.0.1:4321")?;
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