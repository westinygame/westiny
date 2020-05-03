use amethyst::input::{InputHandler, StringBindings};
use amethyst::derive::SystemDesc;
use amethyst::ecs::{Read, System, SystemData, ReadStorage, WriteStorage, ReadExpect};
use amethyst::core::Transform;
use amethyst::core::math::Vector2;

use amethyst::ecs::prelude::Join;
use amethyst::window::ScreenDimensions;

use crate::components::Player;

#[derive(SystemDesc)]
pub struct PlayerMovementSystem;

impl<'s> System<'s> for PlayerMovementSystem {
    type SystemData = (
        WriteStorage<'s, Transform>,
        ReadStorage<'s, Player>,
        Read<'s, InputHandler<StringBindings>>,
        ReadExpect<'s, ScreenDimensions>,
    );

    fn run(&mut self, (mut transforms, players, input, screen): Self::SystemData) {
        for (_player, mut transform) in (&players, &mut transforms).join() {
            rotate_toward_mouse(&mut transform, &input, &screen);

            // TODO walking, ingame position? (rotation with vectors? might be easier for shooting,
            // walking, etc.) or use Tranform for it?
        }
    }

}

fn rotate_toward_mouse(
    transform: &mut Transform,
    input: &InputHandler<StringBindings>,
    screen: &ScreenDimensions
) {
    if let Some((x, y)) = input.mouse_position() {
        // Calculate the vector from middle of screen to mouse cursor
        let mouse_direction = Vector2::new(
            x - screen.width() * 0.5,
            screen.height() * 0.5 - y
        );

        let base_vector = Vector2::new(0.0, -1.0);
        let mut angle = base_vector.angle(&mouse_direction);

        if mouse_direction.x < 0.0 {
            angle = 2.0 * std::f32::consts::PI - angle;
        }

        transform.set_rotation_2d(angle);
    }
}


