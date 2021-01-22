use amethyst::utils::application_root_dir;
use amethyst::{GameDataBuilder, CoreApplication};

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

    let game_data = GameDataBuilder::default();

    let mut game =
        CoreApplication::<_, events::WestinyEvent, events::WestinyEventReader>::build(
        resources_dir,
        states::client_states::ConnectState::default(),
    )?.build(game_data)?;

    log::info!("Starting client");
    game.run();
    Ok(())
}