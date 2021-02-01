use amethyst::prelude::*;
use crate::{
    events::AppEvent,
    systems
};
use amethyst::shred::{Dispatcher, DispatcherBuilder};
use amethyst::core::ecs::WorldExt;
use amethyst::core::ArcThreadPool;
use crate::events::WestinyEvent;
use std::net::SocketAddr;

#[derive(Default)]
pub struct ConnectState {
    dispatcher: Option<Dispatcher<'static, 'static>>,
}

impl State<GameData<'static, 'static>, WestinyEvent> for ConnectState {
    fn on_start(&mut self, data: StateData<'_, GameData<'_, '_>>) {
        let mut world = data.world;

        // TODO get server address from user/config
        let server_addr = ServerAddress { address: Some(SocketAddr::new("127.0.0.1".parse().unwrap(), 4321))};
        world.insert(server_addr);

        let mut dispatcher_builder = DispatcherBuilder::new();

        dispatcher_builder.add(
            systems::client_connect::ClientConnectSystemDesc::default().build(&mut world),
            "client_connect_system", &[]);

        let mut dispatcher = dispatcher_builder
            .with_pool((*world.read_resource::<ArcThreadPool>()).clone())
            .build();
        dispatcher.setup(world);

        self.dispatcher = Some(dispatcher);
    }

    fn handle_event(&mut self, _data: StateData<'_, GameData<'_, '_>>, event: WestinyEvent) -> Trans<GameData<'static, 'static>, WestinyEvent> {
        if let WestinyEvent::App(app_event) = event {
            match app_event {
                AppEvent::Connection(result) => {
                    match result {
                        Ok(init_data) => {
                            log::info!("Initial position: {:?}", init_data.initial_pos);
                            Trans::Switch(Box::new(super::game_states::PlayState::default()))
                        }
                        Err(refuse_cause) => {
                            log::error!("Connection refused. Cause: {}", refuse_cause);
                            Trans::Quit
                        }
                    }
                }
            }
        } else {
            Trans::None
        }
    }

    fn update(&mut self, data: StateData<GameData<'_, '_>>) -> Trans<GameData<'static, 'static>, WestinyEvent> {
        data.data.update(&data.world);
        if let Some(dispatcher) = self.dispatcher.as_mut() {
            dispatcher.dispatch(&data.world);
        }
        Trans::None
    }
}

#[derive(Default)]
pub struct ServerAddress {
    pub address: Option<SocketAddr>
}
