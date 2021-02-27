use amethyst::{
    core::{
        transform::Transform,
        math::{Point2, Vector3},
        ecs::{Dispatcher, DispatcherBuilder},
        SystemBundle,
        ArcThreadPool,
    },
    input::{is_close_requested, is_key_down, VirtualKeyCode, InputBundle},
    prelude::*,
    renderer::Camera,
    window::ScreenDimensions,
};
use std::path::PathBuf;
use amethyst::renderer::SpriteRender;

use westiny_client::MovementBindingTypes;
use westiny_common::resources::{AudioQueue, Seed};
use westiny_client::systems::{
    AudioPlayerSystem,
    NetworkMessageReceiverSystemDesc,
    NetworkEntityStateUpdateSystemDesc,
    NetworkEntityDeleteSystemDesc,
    HudUpdateSystem,
    InputStateSystem,
    CameraMovementSystem,
    CursorPosUpdateSystem,
};
use westiny_common::systems::{HealthUpdateSystemDesc};
use westiny_client::resources::{initialize_audio, initialize_hud, initialize_sprite_resource, SpriteResource};
use westiny_common::{
    events::{AppEvent, WestinyEvent},
    network::ClientInitialData,
};
use westiny_common::resources::map::build_map;
use westiny_common::components::{weapon::Weapon, BoundingCircle};

use crate::entities::{initialize_tilemap, initialize_player};

// later, other states like "MenuState", "PauseState" can be added.
pub struct PlayState {
    dispatcher: Option<Dispatcher<'static, 'static>>,
    resource_dir: PathBuf,
}

impl PlayState {
    pub fn new(resource_dir: &std::path::Path) -> Self {
        PlayState {
            dispatcher: Default::default(),
            resource_dir: resource_dir.to_path_buf(),
        }
    }

    fn place_objects(&self, world: &mut World, seed: Seed) {
        let entities = build_map(world,
                  seed,
                  &self.resource_dir.join("map"))
            .expect("Map could not be created");

        let sprite_resource = world.fetch_mut::<SpriteResource>();
        let mut sprite_storage = world.write_storage::<SpriteRender>();

        entities.iter().for_each(|(entity, sprite_id)| {
            let sprite_render = sprite_resource.sprite_render_for(*sprite_id);
            sprite_storage.insert(*entity, sprite_render).expect("Unable to add sprite to entity during map build");
        })
    }
}

impl State<GameData<'static, 'static>, WestinyEvent> for PlayState {
    fn on_start(&mut self, data: StateData<'_, GameData<'_, '_>>) {
        let mut world = data.world;

        let sprite_resource = initialize_sprite_resource(&mut world);

        let mut dispatcher_builder = DispatcherBuilder::new();

        let key_bindings = self.resource_dir.join("input.ron");

        InputBundle::<MovementBindingTypes>::new().with_bindings_from_file(key_bindings).unwrap()
            .build(&mut world, &mut dispatcher_builder).unwrap();

        let network_message_receiver_sys = NetworkMessageReceiverSystemDesc::default().build(&mut world);
        let network_entity_update_sys = NetworkEntityStateUpdateSystemDesc::default().build(&mut world);
        let entity_delete_system = NetworkEntityDeleteSystemDesc::default().build(&mut world);
        let health_update_system = HealthUpdateSystemDesc::default().build(&mut world);

        let mut dispatcher = dispatcher_builder
            .with(network_message_receiver_sys, "network_message_receiver", &[])
            .with(network_entity_update_sys, "network_entity_update", &[])
            .with(InputStateSystem, "input_state_system", &["input_system"])
            .with(CameraMovementSystem, "camera_movement_system", &["input_system"])
            .with(CursorPosUpdateSystem, "cursor_pos_update_system", &["camera_movement_system"])
            .with(health_update_system, "health_update", &["network_message_receiver"])
            .with(entity_delete_system, "entity_delete", &["network_entity_update"])
            .with(AudioPlayerSystem, "audio_player_system", &["cursor_pos_update_system"])
            .with(HudUpdateSystem, "hud_update_system", &["health_update"])
            .with_pool((*world.read_resource::<ArcThreadPool>()).clone())
            .build();
        dispatcher.setup(world);

        self.dispatcher = Some(dispatcher);

        let dimensions = (*world.read_resource::<ScreenDimensions>()).clone();

        world.insert(AudioQueue::default());

        init_camera(world, &dimensions);

        world.register::<Weapon>();
        world.register::<BoundingCircle>();
        let init_data = (*world.read_resource::<ClientInitialData>()).clone();
        initialize_player(&mut world, &sprite_resource, init_data.player_network_id, init_data.initial_pos);

        initialize_tilemap(world, &sprite_resource, Point2::new(0.0, 0.0));
        initialize_audio(world);
        self.place_objects(&mut world, init_data.seed);
        initialize_hud(world);
    }

    fn handle_event(
        &mut self,
        _data: StateData<'_, GameData<'_, '_>>,
        event: WestinyEvent
    ) -> Trans<GameData<'static, 'static>, WestinyEvent> {
        match event {
            WestinyEvent::EngineEvent(engine_event) => {
                if let StateEvent::Window(event) = engine_event {
                    if is_close_requested(&event) || is_key_down(&event, VirtualKeyCode::Escape) {
                        return Trans::Quit;
                    }
                }
            }
            WestinyEvent::App(app_event) => {
                if let AppEvent::Disconnect = &app_event
                {
                    return Trans::Switch(Box::new(super::connection::ConnectState::new(&self.resource_dir)));
                }
            }
        }

        Trans::None
    }

    fn update(&mut self, data: StateData<GameData<'_, '_>>) -> Trans<GameData<'static, 'static>, WestinyEvent> {
        if let Some(dispatcher) = self.dispatcher.as_mut() {
            dispatcher.dispatch(&data.world);
        }
        data.data.update(&data.world);
        Trans::None
    }

    fn on_stop(&mut self, data: StateData<GameData<'_, '_>>) {
        // This is a quite brute way to wipe out the scene.
        data.world.delete_all();
        data.world.maintain();
    }
}

pub fn init_camera(world: &mut World, dimensions: &ScreenDimensions) {
    let mut transform = Transform::default();
    transform.set_translation_xyz(
        dimensions.width() * 0.5,
        dimensions.height() * 0.5,
        1.0);

    // Zoom-in
    transform.set_scale(Vector3::new(0.25, 0.25, 1.0));

    world
        .create_entity()
        .with(Camera::standard_2d(dimensions.width(), dimensions.height()))
        .with(transform)
        .build();
}
