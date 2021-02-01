use amethyst::{
    assets::{AssetStorage, Loader, Handle},
    core::{
        transform::Transform,
        math::{Point2, Vector3},
        ecs::{Dispatcher, DispatcherBuilder},
        SystemBundle,
        ArcThreadPool,
    },
    input::{is_close_requested, is_key_down, VirtualKeyCode, InputBundle, StringBindings},
    prelude::*,
    renderer::{Camera, ImageFormat, SpriteSheet, SpriteSheetFormat, Texture},
    window::ScreenDimensions,
};
use crate::{
    events::WestinyEvent,
    entities::{initialize_player, initialize_tilemap},
    systems,
};
use std::path::PathBuf;

// later, other states like "MenuState", "PauseState" can be added.
#[derive(Default)]
pub struct PlayState {
    dispatcher: Option<Dispatcher<'static, 'static>>,

    resources_path: PathBuf,
}

impl State<GameData<'static, 'static>, WestinyEvent> for PlayState {
    fn on_start(&mut self, data: StateData<'_, GameData<'_, '_>>) {
        let mut world = data.world;

        let mut dispatcher_builder = DispatcherBuilder::new();

        let key_bindings = self.resources_path.join("input.ron");

        InputBundle::<StringBindings>::new().with_bindings_from_file(key_bindings).unwrap().build(&mut world, &mut dispatcher_builder).unwrap();
        let mut dispatcher = dispatcher_builder
            .with(systems::PlayerMovementSystem, "player_movement_system", &["input_system"])
            .with(systems::CameraMovementSystem, "camera_movement_system", &["player_movement_system"])
            .with(systems::PhysicsSystem, "physics_system", &["player_movement_system"])
            .with(systems::CursorPosUpdateSystem, "cursor_pos_update_system", &["camera_movement_system"])

            .with_pool((*world.read_resource::<ArcThreadPool>()).clone())
            .build();
        dispatcher.setup(world);

        self.dispatcher = Some(dispatcher);

        let dimensions = (*world.read_resource::<ScreenDimensions>()).clone();

        let sprite_sheet_handle = load_sprite_sheet(world);

        init_camera(world, &dimensions);

        let player_init_pos = Point2::new(
            dimensions.width() * 0.5,
            dimensions.height() * 0.5
        );
        initialize_player(world, sprite_sheet_handle.clone(), player_init_pos);
        initialize_tilemap(world, sprite_sheet_handle, Point2::new(dimensions.width() / 2.0, dimensions.height() / 2.0))
    }

    fn handle_event(
        &mut self,
        _data: StateData<'_, GameData<'_, '_>>,
        event: WestinyEvent
    ) -> Trans<GameData<'static, 'static>, WestinyEvent> {
        if let WestinyEvent::EngineEvent(StateEvent::Window(event)) = &event {
            if is_close_requested(event) || is_key_down(&event, VirtualKeyCode::Escape) {
                return Trans::Quit;
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



fn load_sprite_sheet(world: &mut World) -> Handle<SpriteSheet> {
    let texture_handle = {
        let loader = world.read_resource::<Loader>();
        let texture_storage = world.read_resource::<AssetStorage<Texture>>();
        loader.load(
            "spritesheet.png",
            ImageFormat::default(),
            (),
            &texture_storage,
        )
    };
    let loader = world.read_resource::<Loader>();
    let sprite_sheet_store = world.read_resource::<AssetStorage<SpriteSheet>>();
    loader.load(
        "spritesheet.ron",
        SpriteSheetFormat(texture_handle),
        (),
        &sprite_sheet_store,
    )
}
