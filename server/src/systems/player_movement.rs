use westiny_common::MoveDirection;
use westiny_common::components::{Velocity, Input, InputFlags};
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

fn move_directions_from_input(input: &Input) -> Vec<MoveDirection>
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

        let velocity_vec = vector_avg(&velocities);
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
    use westiny_test::assert_delta;
    use std::f32::consts::PI;
    use bevy::prelude::Transform;

    const FACING_UP: f32 = PI;
    const FACING_DOWN: f32 = 0.0;
    const FACING_LEFT: f32 = 3.0*PI/2.0;
    const FACING_RIGHT: f32 = PI/2.0;

    mod test_get_velocity {
        use super::*;

        macro_rules! test_get_velocity {
            ($($name:ident: $player_rotation:expr, $move_dirs:expr, $expected:expr,)*) => {$(
                #[test]
                fn $name() {
                    let mut transform: Transform = Transform::default();
                    transform.rotation = Quat::from_rotation_z($player_rotation);

                    let inputs = $move_dirs;
                    let (exp_x, exp_y) = $expected;

                    let velocity = get_velocity(&transform, &inputs);

                    assert_delta!(exp_x.0, velocity.0.x.0, 0.00001);
                    assert_delta!(exp_y.0, velocity.0.y.0, 0.00001);
                }
            )*}
        }

        use MoveDirection::*;

        test_get_velocity! {
            // forward
            fwd_up: FACING_UP, vec!{Forward}, (MeterPerSec(0.0), PLAYER_MAX_WALK_SPEED),
            fwd_down: FACING_DOWN, vec!{Forward}, (MeterPerSec(0.0), -PLAYER_MAX_WALK_SPEED),
            fwd_left: FACING_LEFT, vec!{Forward}, (-PLAYER_MAX_WALK_SPEED, MeterPerSec(0.0)),
            fwd_right: FACING_RIGHT, vec!{Forward}, (PLAYER_MAX_WALK_SPEED, MeterPerSec(0.0)),
        }
    }
}
