use amethyst::input::{InputHandler, StringBindings};
use amethyst::derive::SystemDesc;
use amethyst::ecs::{Read, System, SystemData, ReadStorage, WriteStorage, ReadExpect};
use amethyst::core::Transform;
use amethyst::core::math::Vector2;

use amethyst::ecs::prelude::Join;
use amethyst::window::ScreenDimensions;

use crate::components::{Player, Velocity};

const ACTION_FORWARD: &str = "forward";
const ACTION_BACKWARD: &str = "backward";
const ACTION_STRAFE_LEFT: &str = "strafe_left";
const ACTION_STRAFE_RIGHT: &str = "strafe_right";
const ACTION_FIRE: &str = "fire";
const ACTION_USE: &str = "use";

#[derive(SystemDesc)]
pub struct PlayerMovementSystem;

pub enum Action {
    Forward,
    Backward,
    StrafeLeft,
    StrafeRight,
    Fire,
    Use,
}

impl Action {
    pub fn get_action_key(&self) -> &str {
        match self {
            Action::Forward => ACTION_FORWARD,
            Action::Backward => ACTION_BACKWARD,
            Action::StrafeLeft => ACTION_STRAFE_LEFT,
            Action::StrafeRight => ACTION_STRAFE_RIGHT,
            Action::Fire => ACTION_FIRE,
            Action::Use => ACTION_USE,
        }
    }

    pub fn from_key(literal: &str) -> Option<Action> {
        match literal {
            ACTION_FORWARD => Some(Action::Forward),
            ACTION_BACKWARD => Some(Action::Backward),
            ACTION_STRAFE_LEFT => Some(Action::StrafeLeft),
            ACTION_STRAFE_RIGHT => Some(Action::StrafeRight),
            ACTION_FIRE => Some(Action::Fire),
            ACTION_USE => Some(Action::Use),
            _ => None,
        }
    }
}

impl<'s> System<'s> for PlayerMovementSystem {
    type SystemData = (
        WriteStorage<'s, Transform>,
        WriteStorage<'s, Velocity>,
        ReadStorage<'s, Player>,
        Read<'s, InputHandler<StringBindings>>,
        ReadExpect<'s, ScreenDimensions>,
    );

    fn run(&mut self, (mut transforms, velocities, players, input, screen): Self::SystemData) {
        for (_player, _velocity, mut transform) in (&players, &velocities, &mut transforms).join() {
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


