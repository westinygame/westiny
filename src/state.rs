use amethyst::{
    core::{
        transform::Transform,
        math::{Point2, Vector3},
    },
    input::{is_close_requested, is_key_down, VirtualKeyCode},
    prelude::*,
    renderer::Camera,
    window::ScreenDimensions,
};

use crate::entities::initialize_player;
use crate::entities::initialize_tilemap;
use crate::resources::initialize_sprite_resource;
use crate::resources::initialize_audio;

// later, other states like "MenuState", "PauseState" can be added.
pub struct PlayState;

impl SimpleState for PlayState {
    fn on_start(&mut self, data: StateData<'_, GameData<'_, '_>>) {
        let world = data.world;
        let dimensions = (*world.read_resource::<ScreenDimensions>()).clone();

        let sprites = initialize_sprite_resource(world);

        init_camera(world, &dimensions);

        let player_init_pos = Point2::new(
            dimensions.width() * 0.5,
            dimensions.height() * 0.5
        );
        initialize_player(world, &sprites, player_init_pos);
        initialize_tilemap(world, &sprites, Point2::new(dimensions.width() / 2.0, dimensions.height() / 2.0));
        initialize_audio(world)
    }

    fn handle_event(
        &mut self,
        _data: StateData<'_, GameData<'_, '_>>,
        event: StateEvent
    ) -> SimpleTrans {

        if let StateEvent::Window(event) = &event {
            if is_close_requested(event) || is_key_down(&event, VirtualKeyCode::Escape) {
                return Trans::Quit;
            }
        }

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
