use crate::events::WestinyEvent;
use amethyst::prelude::*;
use amethyst::core::Time;

use log::info;

#[derive(Default)]
pub struct ServerState;

fn log_fps(time: &Time) {
    if time.frame_number() % 60 == 0 {
        // Note: this is not an average, calculated from the last frame delta.
        info!("FPS: {}", 1.0 / time.delta_real_seconds());
    }
}


impl State<GameData<'static, 'static>, WestinyEvent> for ServerState {
    fn update(&mut self, data: StateData<'_, GameData<'static, 'static>>) -> Trans<GameData<'static, 'static>, WestinyEvent> {
        let time = *data.world.fetch::<Time>();
        log_fps(&time);
        data.data.update(&data.world);
        Trans::None
    }
}
