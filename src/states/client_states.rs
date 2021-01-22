use amethyst::prelude::*;
use crate::{
    events::AppEvent,
    systems
};
use amethyst::shred::{Dispatcher, DispatcherBuilder};
use amethyst::core::ecs::WorldExt;
use amethyst::core::ArcThreadPool;
use crate::events::WestinyEvent;

#[derive(Default)]
pub struct ConnectState {
    dispatcher: Option<Dispatcher<'static, 'static>>,
}

impl State<GameData<'static, 'static>, WestinyEvent> for ConnectState {
    fn on_start(&mut self, data: StateData<'_, GameData<'static, 'static>>) {
        let mut world = data.world;

        let sd = systems::client_connect::ClientConnectSystemDesc::default();
        let sys = sd.build(&mut world);

        let mut dispatcher_builder = DispatcherBuilder::new();
        dispatcher_builder.add(sys, "client_connect_system", &[]);

        let mut dispatcher = dispatcher_builder
            .with_pool((*world.read_resource::<ArcThreadPool>()).clone())
            .build();
        dispatcher.setup(world);

        self.dispatcher = Some(dispatcher);
    }

    fn handle_event(&mut self, _data: StateData<'_, GameData<'_, '_>>, event: WestinyEvent) -> Trans<GameData<'static, 'static>, WestinyEvent> {
        if let WestinyEvent::App(app_event) = event {
            match app_event {
                AppEvent::Connected(ip) => {
                    log::info!("Connection established ({})", ip);

                    // TODO state transition
                    Trans::None
                }
            }
        } else {
            Trans::None
        }
    }

    fn update(&mut self, data: StateData<GameData<'_, '_>>) -> Trans<GameData<'static, 'static>, WestinyEvent> {
        if let Some(dispatcher) = self.dispatcher.as_mut() {
            dispatcher.dispatch(&data.world);
        }
        // call `data.data.update(&data.world);` if you want to run the core systems (defined in `game_data`)
        Trans::None
    }
}
