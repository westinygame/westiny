use amethyst::input::{InputHandler, StringBindings};
use amethyst::derive::SystemDesc;
use amethyst::ecs::{Read, System, SystemData, ReadStorage, WriteStorage, ReadExpect, WriteExpect};
use amethyst::ecs::prelude::Join;
use amethyst::core::Transform;
use amethyst::core::math::{Point2, Vector2};
use amethyst::core::geometry::Plane;

use amethyst::window::ScreenDimensions;
use amethyst::renderer::camera::Camera;
use crate::resources::CursorPosition;


#[derive(SystemDesc)]
pub struct CursorPosUpdateSystem;

impl<'s> System<'s> for CursorPosUpdateSystem {
    type SystemData = (
        WriteStorage<'s, Transform>,
        ReadStorage<'s, Camera>,
        Read<'s, InputHandler<StringBindings>>,
        ReadExpect<'s, ScreenDimensions>,
        WriteExpect<'s, CursorPosition>
    );

    fn run(&mut self, (mut transforms, cameras, input, screen_dimensions, mut cursor_pos): Self::SystemData) {
        for (camera, transform) in (&cameras, &mut transforms).join() {
            if let Some((mouse_x, mouse_y)) = input.mouse_position() {
                let ray = camera.projection().screen_ray(
                    Point2::new(mouse_x, mouse_y),
                    Vector2::new(screen_dimensions.width(), screen_dimensions.height()),
                    transform,
                );
                let distance = ray.intersect_plane(&Plane::with_z(0.0)).unwrap();
                let mouse_world_position = ray.at_distance(distance);

                *cursor_pos = CursorPosition{ pos: mouse_world_position.xy()};
            }
        }
    }
}
