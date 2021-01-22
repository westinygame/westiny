use crate::events::WestinyEvent;
use amethyst::prelude::*;

#[derive(Default)]
pub struct ServerState;

impl State<GameData<'static, 'static>, WestinyEvent> for ServerState {
    fn update(&mut self, data: StateData<'_, GameData<'static, 'static>>) -> Trans<GameData<'static, 'static>, WestinyEvent> {
        data.data.update(&data.world);
        Trans::None
    }
}