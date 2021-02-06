use crate::events::WestinyEvent;
use amethyst::prelude::*;
use amethyst::core::Time;
use westiny_server::resources::ClientRegistry;

use log::info;

#[derive(Default)]
pub struct ServerState;

fn log_fps(time: &Time) {
    if time.frame_number() % 60 == 0 {
        // Note: this is not an average, calculated from the last frame delta.
        info!("FPS: {}", 1.0 / time.delta_real_seconds());
    }
}


fn log_clients(time: &Time, registry: &ClientRegistry) {
    if time.frame_number() % 600 == 0 {
        log::info!("Number of players online: {}", registry.client_count());
        log::debug!("{}", &registry);
    }
}


impl State<GameData<'static, 'static>, WestinyEvent> for ServerState {
    fn on_start(&mut self, data: StateData<'_, GameData<'static, 'static>>) {
        data.world.insert(ClientRegistry::new(16));
    }

    fn update(&mut self, data: StateData<'_, GameData<'static, 'static>>) -> Trans<GameData<'static, 'static>, WestinyEvent> {
        let time = *data.world.fetch::<Time>();
        log_fps(&time);
        data.data.update(&data.world);
        log_clients(&time, &data.world.fetch::<ClientRegistry>());
        Trans::None
    }
}
