use amethyst::prelude::*;
use crate::{
    events::{AppEvent, WestinyEvent},
    systems,
    utilities::*,
};
use amethyst::shred::{Dispatcher, DispatcherBuilder};
use amethyst::core::ecs::WorldExt;
use amethyst::core::ArcThreadPool;
use westiny_common::resources::ServerAddress;
use crate::resources;
use westiny_common::components::{NetworkId, Player, Velocity, BoundingCircle};
use amethyst::core::math::Point2;
use westiny_common::components::weapon::Weapon;

pub struct ConnectState {
    dispatcher: Option<Dispatcher<'static, 'static>>,
    resource_dir: std::path::PathBuf,
    sprite_resource: Option<resources::SpriteResource>,
}

impl ConnectState {
    pub fn new(resource_dir: &std::path::Path) -> Self {
        ConnectState {
            dispatcher: Default::default(),
            resource_dir: resource_dir.to_path_buf(),
            sprite_resource: None,
        }
    }
}

impl State<GameData<'static, 'static>, WestinyEvent> for ConnectState {
    fn on_start(&mut self, data: StateData<'_, GameData<'_, '_>>) {
        let mut world = data.world;

        let server_address: ServerAddress = {
            let ron_path = self.resource_dir.join("server_network.ron");
            read_ron(&ron_path).unwrap_or_else(|err| {
                let address = ServerAddress::default();
                log::warn!("Failed to read server network configuration file: {}, error: [{}] \
                Using default server port ({})",
                           ron_path.as_os_str().to_str().unwrap(),
                           err,
                           address.address);
                address
            })
        };
        world.insert(server_address);

        // Must be registered here because no system uses them in this dispathcer
        world.register::<NetworkId>();
        world.register::<Player>();
        world.register::<Velocity>();
        world.register::<BoundingCircle>();
        world.register::<Weapon>();

        self.sprite_resource = Some(resources::initialize_sprite_resource(&mut world));

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
                            crate::entities::initialize_player(
                                data.world,
                                &self.sprite_resource.as_ref().unwrap(),
                                init_data.player_network_id,
                                Point2::from([0.0, 0.0]));
                            Trans::Switch(Box::new(super::game_states::PlayState::new(&self.resource_dir, self.sprite_resource.clone().unwrap())))
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




