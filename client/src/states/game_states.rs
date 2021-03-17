use amethyst::{
    core::{
        transform::Transform,
        math::{Point2, Vector3},
        ecs::{Dispatcher, DispatcherBuilder},
        ArcThreadPool,
    },
    input::{is_close_requested, is_key_down, VirtualKeyCode},
    prelude::*,
    renderer::Camera,
    window::ScreenDimensions,
};
use std::path::PathBuf;
use amethyst::renderer::SpriteRender;

use crate::systems::{
    AudioPlayerSystem,
    NetworkMessageReceiverSystemDesc,
    NetworkEntityStateUpdateSystemDesc,
    NetworkEntityDeleteSystemDesc,
    HudUpdateSystem,
    NotificationBarSystemDesc,
    InputStateSystem,
    CameraMovementSystem,
    CursorPosUpdateSystem,
    PhysicsSystem,
    ShooterSystemDesc,
    LifespanSystem,
    HealthUpdateSystemDesc,
    CollisionBundle,
};
use crate::resources::{initialize_audio, initialize_hud, NotificationBar, initialize_sprite_resource, SpriteResource, PlayerNetworkId};
use crate::entities::initialize_tilemap;

use westiny_common::{
    components::BoundingCircle,
    events::{AppEvent, WestinyEvent},
    network::ClientInitialData,
    resources::{AudioQueue, Seed, map::build_map}
};
use westiny_common::components::Projectile;
use amethyst::core::SystemBundle;

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

        // TODO remove when collision is turned on
        world.register::<Projectile>();
        let mut dispatcher_builder = DispatcherBuilder::new();

        let network_message_receiver_sys = NetworkMessageReceiverSystemDesc::default().build(&mut world);
        let network_entity_update_sys = NetworkEntityStateUpdateSystemDesc::default().build(&mut world);
        let entity_delete_system = NetworkEntityDeleteSystemDesc::default().build(&mut world);
        let health_update_system = HealthUpdateSystemDesc::default().build(&mut world);
        let notification_bar_sys = NotificationBarSystemDesc::default().build(&mut world);
        let shooter_system = ShooterSystemDesc::default().build(&mut world);

        dispatcher_builder = dispatcher_builder
            .with(network_message_receiver_sys, "network_message_receiver", &[])
            .with(network_entity_update_sys, "network_entity_update", &[])
            .with(CameraMovementSystem, "camera_movement_system", &["network_entity_update"])
            .with(CursorPosUpdateSystem, "cursor_pos_update_system", &["camera_movement_system"])
            .with(InputStateSystem, "input_state_system", &["cursor_pos_update_system"])
            .with(PhysicsSystem, "physics", &[])
            .with(health_update_system, "health_update", &["network_message_receiver"])
            .with(shooter_system, "shooter", &["network_message_receiver"])
            .with(LifespanSystem, "lifespan", &["shooter"])
            .with(AudioPlayerSystem, "audio_player_system", &["cursor_pos_update_system"])
            .with(HudUpdateSystem, "hud_update_system", &["health_update"])
            .with(notification_bar_sys, "notification_bar", &["network_message_receiver"])
            .with_pool((*world.read_resource::<ArcThreadPool>()).clone());

        CollisionBundle.build(world, &mut dispatcher_builder).expect("Unable to build CollisionBundle");
        dispatcher_builder.add(entity_delete_system, "entitiy_delete", &["network_entity_update", "projectile_collision_handler"]);

        let mut dispatcher= dispatcher_builder.build();
        dispatcher.setup(world);

        self.dispatcher = Some(dispatcher);

        let dimensions = (*world.read_resource::<ScreenDimensions>()).clone();

        world.insert(AudioQueue::default());

        init_camera(world, &dimensions);

        let init_data = (*world.read_resource::<ClientInitialData>()).clone();
        world.insert(PlayerNetworkId(init_data.player_network_id));

        initialize_tilemap(world, &sprite_resource, Point2::new(0.0, 0.0));
        initialize_audio(world);

        world.register::<BoundingCircle>();
        self.place_objects(&mut world, init_data.seed);
        initialize_hud(&mut world);
        NotificationBar::initialize(&mut world);
    }

    fn on_stop(&mut self, data: StateData<GameData<'_, '_>>) {
        // This is a quite brute way to wipe out the scene.
        data.world.delete_all();
        data.world.maintain();
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
        data.data.update(&data.world);
        if let Some(dispatcher) = self.dispatcher.as_mut() {
            dispatcher.dispatch(&data.world);
        }
        Trans::None
    }
}

const CAMERA_ALTITUDE: f32 = 3.0;
const CAMERA_DEPTH_VISION: f32 = CAMERA_ALTITUDE + 1.0;

fn create_camera(width: f32, height: f32) -> Camera {
    Camera::orthographic(
        -width / 2.0,
        width / 2.0,
        -height / 2.0,
        height / 2.0,
        0.125, // minimum distance from camera
        CAMERA_DEPTH_VISION,
        )
}

pub fn init_camera(world: &mut World, dimensions: &ScreenDimensions) {
    let mut transform = Transform::default();
    transform.set_translation_xyz(
        dimensions.width() * 0.5,
        dimensions.height() * 0.5,
        CAMERA_ALTITUDE);

    // Zoom-in
    transform.set_scale(Vector3::new(0.25, 0.25, 1.0));

    world
        .create_entity()
        .with(create_camera(dimensions.width(), dimensions.height()))
        .with(transform)
        .build();
}
