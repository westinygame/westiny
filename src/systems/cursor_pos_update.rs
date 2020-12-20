use amethyst::input::{InputHandler, StringBindings};
use amethyst::derive::SystemDesc;
use amethyst::ecs::{Read, System, SystemData, ReadStorage, ReadExpect, WriteExpect};
use amethyst::ecs::prelude::Join;
use amethyst::core::Transform;
use amethyst::core::math::{Point2, Vector2};
use amethyst::core::geometry::Plane;

use amethyst::window::ScreenDimensions;
use amethyst::renderer::camera::Camera;
use crate::resources::CursorPosition;


#[derive(SystemDesc, Default)]
pub struct CursorPosUpdateSystem;

impl<'s> System<'s> for CursorPosUpdateSystem {
    type SystemData = (
        ReadStorage<'s, Transform>,
        ReadStorage<'s, Camera>,
        Read<'s, InputHandler<StringBindings>>,
        ReadExpect<'s, ScreenDimensions>,
        WriteExpect<'s, CursorPosition>
    );

    fn run(&mut self, (transforms, cameras, input, screen_dimensions, mut cursor_pos): Self::SystemData) {
        for (camera, transform) in (&cameras, &transforms).join() {
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


#[cfg(test)]
mod test_integration {
    use super::*;

    use amethyst::Error;
    use amethyst::prelude::*;
    use amethyst_test::prelude::*;
    use amethyst::input::StringBindings;
    use crate::test_helpers as helper;
    use crate::state::init_camera;

    #[test]
    fn simple_cursor_position_update() -> Result<(), Error> {
        AmethystApplication::ui_base::<StringBindings>()
            .with_system(CursorPosUpdateSystem::default(), "cursor_pos_update_system", &[])
            .with_setup(|world| {
                // resources must be added during setup
                world.insert(CursorPosition::default());
            })
            .with_effect(|world| {
                let dimensions = (*world.read_resource::<ScreenDimensions>()).clone();
                init_camera(world, &dimensions);

                let event = helper::make_cursor_moved(
                    (dimensions.width() / 2.0).into(),
                    (dimensions.height() / 2.0).into()
                );
                helper::send_input_event(event, &world);
            })
            .with_assertion(|world| {
                let cursor = world.read_resource::<CursorPosition>();

                assert_eq!(cursor.pos.x, 400.0);
                assert_eq!(cursor.pos.y, 300.0);
            })
            .run()
    }
}
