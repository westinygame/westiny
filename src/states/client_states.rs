use amethyst::prelude::*;
use crate::{
    events::AppEvent,
    systems
};
use amethyst::shred::{Dispatcher, DispatcherBuilder};
use amethyst::core::ecs::WorldExt;
use amethyst::core::{ArcThreadPool, SystemBundle};
use crate::events::WestinyEvent;
use amethyst::network::simulation::laminar::{LaminarNetworkBundle, LaminarSocket};
use std::net::SocketAddr;

#[derive(Default)]
pub struct ConnectState {
    dispatcher: Option<Dispatcher<'static, 'static>>,
}

impl State<GameData<'static, 'static>, WestinyEvent> for ConnectState {
    fn on_start(&mut self, data: StateData<'_, GameData<'static, 'static>>) {
        let mut world = data.world;

        // TODO get server address from user/config
        let server_addr = ServerAddress { address: Some(SocketAddr::new("127.0.0.1".parse().unwrap(), 4321))};
        world.insert(server_addr);

        let mut dispatcher_builder = DispatcherBuilder::new();

        let socket = LaminarSocket::bind("127.0.0.1:1234").unwrap();
        LaminarNetworkBundle::new(Some(socket)).build(&mut world, &mut dispatcher_builder).unwrap();

        dispatcher_builder.add(
            systems::client_connect::ClientConnectSystemDesc::default().build(&mut world),
            "client_connect_system",
            &["network_recv"]);

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
                    Trans::Quit
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

#[derive(Default)]
pub struct ServerAddress {
    pub address: Option<SocketAddr>
}
