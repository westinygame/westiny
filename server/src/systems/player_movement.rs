use westiny_common::MoveDirection;
use westiny_common::components::{Player, Velocity, Input, InputFlags};
use westiny_common::utilities::set_rotation_toward_vector;
use westiny_common::metric_dimension::{MeterPerSecVec2, MeterPerSec};
use bevy::prelude::*;

pub fn apply_input(mut query: Query<(&mut Transform, &mut Velocity, &Input)>) {
    for (mut transform, mut velocity, input) in query.iter_mut() {
        set_rotation_toward_vector(&mut transform, &input.cursor.into_pixel_vec());

        let move_inputs = move_directions_from_input(&input);
        *velocity = get_velocity(&transform, &move_inputs);
    }
}

pub fn move_directions_from_input(input: &Input) -> Vec<MoveDirection>
{
    let mut directions = Vec::new();
    if input.flags.intersects(InputFlags::FORWARD)
    {
        directions.push(MoveDirection::Forward);
    }
    if input.flags.intersects(InputFlags::BACKWARD)
    {
        directions.push(MoveDirection::Backward);
    }
    if input.flags.intersects(InputFlags::LEFT)
    {
        directions.push(MoveDirection::StrafeLeft);
    }
    if input.flags.intersects(InputFlags::RIGHT)
    {
        directions.push(MoveDirection::StrafeRight);
    }
    directions
}


const PLAYER_MAX_WALK_SPEED: MeterPerSec = MeterPerSec(4.0);

// TODO It would be better to use a more generic IntoIterator instead of the specific vector type.
fn get_velocity (transform: &Transform,
                 move_inputs: &Vec<MoveDirection>) -> Velocity {
    if move_inputs.is_empty() {
        Velocity::default()
    } else {
        let velocities: Vec<MeterPerSecVec2> = move_inputs.into_iter()
            .map(|dir| as_vector2(*dir))
            .collect();

        let mut velocity_vec = vector_avg(&velocities);
        Velocity(velocity_vec.rotate(&transform.rotation))
    }
}

fn vector_avg<'a, I>(velocities: I) -> MeterPerSecVec2
    where I: IntoIterator<Item=&'a MeterPerSecVec2> {

    let mut sum_vec = MeterPerSecVec2::from_raw(0.0, 0.0);
    let mut len = 0;

    for &vel in velocities {
        sum_vec += vel;
        len += 1;
    }

    sum_vec / (len as f32)
}

fn as_vector2(move_dir: MoveDirection) -> MeterPerSecVec2 {
    match move_dir {
        MoveDirection::Forward     => MeterPerSecVec2 {
                                          x: MeterPerSec(0.0),
                                          y: -PLAYER_MAX_WALK_SPEED
                                      },
        MoveDirection::Backward    => MeterPerSecVec2 {
                                          x: MeterPerSec(0.0),
                                          y: PLAYER_MAX_WALK_SPEED / 2.0
                                      },
        MoveDirection::StrafeLeft  => MeterPerSecVec2 {
                                          x: PLAYER_MAX_WALK_SPEED / 2.0,
                                          y: MeterPerSec(0.0)
                                      },
        MoveDirection::StrafeRight => MeterPerSecVec2 {
                                          x: -PLAYER_MAX_WALK_SPEED / 2.0,
                                          y: MeterPerSec(0.0)
                                      }
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use westiny_test::f32_eq;
    use std::f32::consts::PI;
    use amethyst::core::Transform;

    const FACING_UP: f32 = PI;
    const FACING_DOWN: f32 = 0.0;
    const FACING_LEFT: f32 = -PI/2.0;
    const FACING_RIGHT: f32 = PI/2.0;

    mod test_rotate_toward_mouse {
        use super::*;

        macro_rules! test_rotate_toward_mouse {
            ($($name:ident: $player_coord:expr, $cursor_coord:expr, $expected:expr,)*) => {
                $(
                    #[test]
                    fn $name() {
                        let player = $player_coord;
                        let mut transform = &mut Transform::default();
                        transform.set_translation_x(player.0);
                        transform.set_translation_y(player.1);

                        let cursor_pos = Point2::new($cursor_coord.0, $cursor_coord.1);

                        rotate_toward_point(&mut transform, &cursor_pos);

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
        use westiny_test::f32_eq;

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

                    assert!(f32_eq(exp_x.0, velocity.0.x.0), "velocity x -> Expected: {}, Actual: {}", exp_x, velocity.0.x);
                    assert!(f32_eq(exp_y.0, velocity.0.y.0), "velocity y -> Expected: {}, Actual: {}", exp_y, velocity.0.y);
                }
            )*}
        }

        use MoveDirection::*;

        test_update_velocity! {
            // forward
            fwd_up: FACING_UP, vec!{Forward}, (MeterPerSec(0.0), PLAYER_MAX_WALK_SPEED),
            fwd_down: FACING_DOWN, vec!{Forward}, (MeterPerSec(0.0), -PLAYER_MAX_WALK_SPEED),
            fwd_left: FACING_LEFT, vec!{Forward}, (-PLAYER_MAX_WALK_SPEED, MeterPerSec(0.0)),
            fwd_right: FACING_RIGHT, vec!{Forward}, (PLAYER_MAX_WALK_SPEED, MeterPerSec(0.0)),
        }
    }

}
