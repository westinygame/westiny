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

use crate::resources::{SpriteResource, SpriteId};
use crate::components::BoundingCircle;
use crate::components::Projectile;
use crate::resources::{Collisions, ProjectileCollisions};
use crate::resources::SoundPlayer;

// later, other states like "MenuState", "PauseState" can be added.
pub struct PlayState;

impl SimpleState for PlayState {
    fn on_start(&mut self, data: StateData<'_, GameData<'_, '_>>) {
        let world = data.world;
        let dimensions = (*world.read_resource::<ScreenDimensions>()).clone();

        let sprites = initialize_sprite_resource(world);

        world.insert(Collisions::default());
        world.insert(ProjectileCollisions::default());
        world.insert(SoundPlayer::default());
        init_camera(world, &dimensions);

        let player_init_pos = Point2::new(
            dimensions.width() * 0.5,
            dimensions.height() * 0.5
        );
        initialize_player(world, &sprites, player_init_pos);
        initialize_tilemap(world, &sprites, Point2::new(dimensions.width() / 2.0, dimensions.height() / 2.0));
        initialize_audio(world);
        place_objects(world, &sprites, player_init_pos);
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

fn place_objects(world: &mut World, sprites: &SpriteResource, player_init_pos: Point2<f32>) {
    //TODO placing barrels and other objects should be based on a map
    place_barrel(world, &sprites, player_init_pos, 3, 3);
    place_barrel(world, &sprites, player_init_pos, 3, 5);
    place_barrel(world, &sprites, player_init_pos, 3, 6);
    place_barrel(world, &sprites, player_init_pos, 3, 7);
    place_barrel(world, &sprites, player_init_pos, 3, 8);
    place_barrel(world, &sprites, player_init_pos, 4, 8);
    place_barrel(world, &sprites, player_init_pos, 5, 8);
    place_barrel(world, &sprites, player_init_pos, 5, 7);
}

fn place_barrel(world: &mut World, sprites: &SpriteResource, player_init_pos: Point2<f32>, x: u32, y: u32) {

    let mut transform = Transform::default();
    transform.set_translation_xyz(player_init_pos.x + (x as f32) * 16.0, player_init_pos.y + (y as f32) * 16.0, 0.0);

    world
        .create_entity()
        .with(sprites.sprite_render_for(SpriteId::Barrel))
        .with(transform)
        .with(BoundingCircle{radius: 8.0})
        .build();
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
