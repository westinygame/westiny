use amethyst::prelude::*;
use amethyst::shred::{Dispatcher, DispatcherBuilder};
use amethyst::core::ecs::WorldExt;
use amethyst::core::ArcThreadPool;
use westiny_common::{
    resources::ServerAddress,
    events::{AppEvent, WestinyEvent},
};
use crate::systems;
use std::net::SocketAddr;
use std::str::FromStr;

pub struct ConnectState {
    dispatcher: Option<Dispatcher<'static, 'static>>,
    resource_dir: std::path::PathBuf,
}

impl ConnectState {
    pub fn new(resource_dir: &std::path::Path) -> Self {
        ConnectState {
            dispatcher: Default::default(),
            resource_dir: resource_dir.to_path_buf(),
        }
    }
}

impl State<GameData<'static, 'static>, WestinyEvent> for ConnectState {
    fn on_start(&mut self, data: StateData<'_, GameData<'_, '_>>) {
        let mut world = data.world;

        world.insert(get_server_address());

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

    fn handle_event(&mut self, data: StateData<'_, GameData<'_, '_>>, event: WestinyEvent) -> Trans<GameData<'static, 'static>, WestinyEvent> {
        if let WestinyEvent::App(app_event) = event {
            match app_event {
                AppEvent::Connection(result) => {
                    match result {
                        Ok(init_data) => {
                            // Put init data here thus PlayState will be able to fetch it
                            data.world.insert(init_data);
                            Trans::Switch(Box::new(super::game_states::PlayState::new(&self.resource_dir, )))
                        }
                        Err(refuse_cause) => {
                            log::error!("Connection refused. Cause: {}", refuse_cause);
                            Trans::Quit
                        }
                    }
                }
                AppEvent::Disconnect => {
                    log::error!("Invalid Disconnect event received in ConnectState");
                    Trans::Quit
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

fn get_server_address() -> ServerAddress {
    let address_result = std::env::var("WESTINY_SERVER_ADDRESS")
        .map_err(|err| {
            anyhow::Error::from(err)
        })
        .and_then(|env| SocketAddr::from_str(&env)
            .map_err(|err|anyhow::Error::from(err)))
        .map(|addr| ServerAddress { address:addr });

    match address_result {
        Ok(addr) => {
            log::info!("Server address: {}", addr.address);
            addr
        }
        Err(err) => {
            let addr = ServerAddress::default();
            log::warn!("Server address has not been configured. Error: {}. Using default address: {}", err, addr.address);
            addr
        }
    }
}




