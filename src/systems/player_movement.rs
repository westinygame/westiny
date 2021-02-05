use std::fmt;

use amethyst::input::{InputHandler, BindingTypes};
use amethyst::derive::SystemDesc;
use amethyst::ecs::{Read, System, SystemData, ReadStorage, WriteStorage, prelude::Join};
use amethyst::core::Transform;
use amethyst::core::math::{Vector2, Rotation2, Point2};
use serde::{Serialize, Deserialize};

use crate::components::{Player, Velocity};
use crate::resources::CursorPosition;

use westiny_common::MoveDirection;


#[derive(Clone, Debug, Hash, PartialEq, Eq, Serialize, Deserialize)]
pub enum ActionBinding {
    Forward,
    Backward,
    StrafeLeft,
    StrafeRight,
    Shoot,
    Use,
}

#[derive(Clone, Debug, Hash, PartialEq, Eq, Serialize, Deserialize)]
pub enum AxisBinding {
    Zoom
}

impl fmt::Display for ActionBinding {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}

impl fmt::Display for AxisBinding {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}


#[derive(Debug)]
pub struct MovementBindingTypes;

impl BindingTypes for MovementBindingTypes {
    type Axis = AxisBinding;
    type Action = ActionBinding;
}

const MOVE_ACTIONS: &'static [&'static ActionBinding] = &[
    &ActionBinding::Forward,
    &ActionBinding::Backward,
    &ActionBinding::StrafeLeft,
    &ActionBinding::StrafeRight,
];

#[derive(SystemDesc)]
pub struct PlayerMovementSystem;

pub fn move_direction_from_binding(binding: &ActionBinding) -> Option<MoveDirection> {
    match binding {
        ActionBinding::Forward => Some(MoveDirection::Forward),
        ActionBinding::Backward => Some(MoveDirection::Backward),
        ActionBinding::StrafeLeft => Some(MoveDirection::StrafeLeft),
        ActionBinding::StrafeRight => Some(MoveDirection::StrafeRight),
        _ => None,
    }
}

impl<'s> System<'s> for PlayerMovementSystem {
    type SystemData = (
        WriteStorage<'s, Transform>,
        WriteStorage<'s, Velocity>,
        ReadStorage<'s, Player>,
        Read<'s, InputHandler<MovementBindingTypes>>,
        Read<'s, CursorPosition>
    );

    fn run(&mut self, (mut transforms, mut velocities, players, input, cursor_pos): Self::SystemData) {

        for (_player, mut velocity, mut transform) in (&players, &mut velocities, &mut transforms).join() {
            rotate_toward_mouse(&mut transform, &cursor_pos.pos);

            let move_inputs: Vec<MoveDirection> = MOVE_ACTIONS.iter()
                .filter(|s| input.action_is_down(&s).unwrap_or(false))
                .filter_map(|&s| move_direction_from_binding(s))
                .collect();

            update_velocity(&transform, &move_inputs, &mut velocity);
        }
    }
}

fn rotate_toward_mouse(
    transform: &mut Transform,
    cursor_pos: &Point2<f32>,
) {
    // Calculate the vector from player position to mouse cursor
    let mouse_direction = cursor_pos.to_homogeneous() - transform.translation();

    let base_vector = Vector2::new(0.0, -1.0);
    let mut angle = base_vector.angle(&mouse_direction.xy());

    if mouse_direction.x < 0.0 {
        angle = 2.0 * std::f32::consts::PI - angle;
    }
    transform.set_rotation_2d(angle);
}

const PLAYER_MAX_WALK_SPEED: f32 = 64.0;

// TODO It would be better to use a more generic IntoIterator instead of the specific vector type.
// I did not manage to call into_iter on <T: IntoIterator<Item=MoveDirection>> type
fn update_velocity(
    transform: &Transform,
    move_inputs: &Vec<MoveDirection>,
    velocity: &mut Velocity
) {
    *velocity = if move_inputs.is_empty() {
        Velocity::default()
    } else {
        let velocities: Vec<Vector2<f32>> = move_inputs.into_iter()
            .map(|dir| as_vector2(*dir))
            .collect();

        let angle = transform.rotation().axis().map(|vec| vec.z).unwrap_or(1.0) * transform.rotation().angle();
        let rot = Rotation2::new(angle);
        Velocity(rot * vector_avg(&velocities))
    };
}

fn vector_avg<'a, I>(velocities: I) -> Vector2<f32>
    where I: IntoIterator<Item=&'a Vector2<f32>> {
    let mut x = 0_f32;
    let mut y = 0_f32;
    let mut len = 0;

    for &vel in velocities {
        x += vel.x;
        y += vel.y;
        len += 1;
    }

    Vector2::new(x/len as f32, y/len as f32)

}

// TODO I couldn't manage to create valid rustdoc links :(
/// Gives the corresponding `Vector2` to the given `MoveDirection` element.
/// In te case of `Forward` the length of the returned vector will be the max walk speed
/// and the halt of that in any other cases
fn as_vector2(move_dir: MoveDirection) -> Vector2<f32> {
    match move_dir {
        MoveDirection::Forward => Vector2::new(0.0, -PLAYER_MAX_WALK_SPEED),
        MoveDirection::Backward => Vector2::new(0.0, PLAYER_MAX_WALK_SPEED / 2.0),
        MoveDirection::StrafeLeft => Vector2::new(PLAYER_MAX_WALK_SPEED / 2.0, 0.0),
        MoveDirection::StrafeRight => Vector2::new(-PLAYER_MAX_WALK_SPEED / 2.0, 0.0)
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use std::f32::consts::PI;
    use amethyst::core::Transform;

    const FACING_UP: f32 = PI;
    const FACING_DOWN: f32 = 0.0;
    const FACING_LEFT: f32 = -PI/2.0;
    const FACING_RIGHT: f32 = PI/2.0;

    #[inline]
    fn f32_eq(f1: f32, f2: f32) -> bool {
        const F32_ALLOWED_DIFF: f32 = 0.00001;
        (f1 - f2).abs() < F32_ALLOWED_DIFF
    }

    mod test_rotate_toward_mouse {
        use super::*;

        macro_rules! test_rotate_toward_mouse {
            ($($name:ident: $player_coord:expr, $cursor_coord:expr, $expected:expr,)*) => {
                $(
                    #[test]
                    fn $name() {
                        let player = $player_coord;
                        let transform = &mut Transform::default();
                        transform.set_translation_x(player.0);
                        transform.set_translation_y(player.1);

                        let cursor_pos = Point2::new($cursor_coord.0, $cursor_coord.1);

                        rotate_toward_mouse(transform, &cursor_pos);

                        let angle = transform.rotation().axis().map(|vec| vec.z).unwrap_or(1.0) * transform.rotation().angle();

                        // sin is called to normalize the angles (e.g. -PI = PI)
                        assert!(f32_eq(f32::sin($expected), angle.sin()), "Expected angle: {}, Actual angle: {}", $expected, angle);
                    }
                )*
            }
        }

        test_rotate_toward_mouse! {
            cursor_up: (3.0, 3.0), (3.0, 2.0), FACING_UP,
            cursor_left: (3.0, 3.0), (2.0, 3.0), FACING_LEFT,
            cursor_down: (3.0, 3.0), (3.0, 4.0), FACING_DOWN,
            cursor_right: (3.0, 3.0), (4.0, 3.0), FACING_RIGHT,
            cursor_upright_45deg: (3.0, 3.0), (4.0, 2.0), 3.0*PI/4.0,
        }
    }

    mod test_update_velocity {
        use super::*;

        macro_rules! test_update_velocity {
            ($($name:ident: $player_rotation:expr, $move_dirs:expr, $expected:expr,)*) => {$(
                #[test]
                fn $name() {
                    let mut transform: Transform = Transform::default();
                    transform.set_rotation_2d($player_rotation);

                    let inputs = $move_dirs;
                    let (exp_x, exp_y) = $expected;

                    let mut velocity = Velocity::default();
                    update_velocity(&transform, &inputs, &mut velocity);

                    assert!(f32_eq(exp_x, velocity.0.x), "velocity x -> Expected: {}, Actual: {}", exp_x, velocity.0.x);
                    assert!(f32_eq(exp_y, velocity.0.y), "velocity y -> Expected: {}, Actual: {}", exp_y, velocity.0.y);
                }
            )*}
        }

        use MoveDirection::*;

        test_update_velocity! {
            // forward
            fwd_up: FACING_UP, vec!{Forward}, (0.0, PLAYER_MAX_WALK_SPEED),
            fwd_down: FACING_DOWN, vec!{Forward}, (0.0, -PLAYER_MAX_WALK_SPEED),
            fwd_left: FACING_LEFT, vec!{Forward}, (-PLAYER_MAX_WALK_SPEED, 0.0),
            fwd_right: FACING_RIGHT, vec!{Forward}, (PLAYER_MAX_WALK_SPEED, 0.0),
        }
    }

}
