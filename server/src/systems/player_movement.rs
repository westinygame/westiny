use amethyst::derive::SystemDesc;
use amethyst::ecs::{System, SystemData, ReadStorage, WriteStorage, Join};
use amethyst::core::Transform;
use amethyst::core::math::{Vector2, Rotation2, Point2};

use westiny_common::MoveDirection;
use westiny_common::components::{Player, Velocity};
use westiny_common::components::{InputFlags, Input};
use westiny_common::metric_dimension::{MeterPerSec, rotate};
use amethyst::core::num::Zero;

#[derive(SystemDesc)]
pub struct PlayerMovementSystem;

impl<'s> System<'s> for PlayerMovementSystem {
    type SystemData = (
        WriteStorage<'s, Transform>,
        WriteStorage<'s, Velocity>,
        ReadStorage<'s, Player>,
        ReadStorage<'s, Input>,
    );

    fn run(&mut self, (mut transforms, mut velocities, players, inputs): Self::SystemData) {
        for (_player, input, mut velocity, transform) in (&players, &inputs, &mut velocities, &mut transforms).join() {
            rotate_toward_point(transform, &Point2::new(input.cursor.x.into_pixel(), input.cursor.y.into_pixel()));

            let move_inputs = move_directions_from_input(&input);
            log::debug!("{:?} {}", input, move_inputs.len());

            update_velocity(&transform, &move_inputs, &mut velocity);
        }
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


pub fn rotate_toward_point(
    transform: &mut Transform,
    point: &Point2<f32>
) {
    use westiny_common::utilities::set_rotation_toward_vector;

    // Calculate the vector from player position to mouse cursor
    let direction: Vector2<f32> = (point.to_homogeneous() - transform.translation()).xy();
    set_rotation_toward_vector(transform, &direction);
}

const PLAYER_MAX_WALK_SPEED: MeterPerSec = MeterPerSec(4.0);

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
        let velocities: Vec<Vector2<MeterPerSec>> = move_inputs.into_iter()
            .map(|dir| as_vector2(*dir))
            .collect();

        let angle = transform.rotation().axis().map(|vec| vec.z).unwrap_or(1.0) * transform.rotation().angle();
        let rot = Rotation2::new(angle);
        Velocity(rotate(vector_avg(&velocities), rot))
    };
}

fn vector_avg<'a, I>(velocities: I) -> Vector2<MeterPerSec>
    where I: IntoIterator<Item=&'a Vector2<MeterPerSec>> {

    let mut x = MeterPerSec::zero();
    let mut y = MeterPerSec::zero();
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
/// and the half of that in any other cases
fn as_vector2(move_dir: MoveDirection) -> Vector2<MeterPerSec> {
    match move_dir {
        MoveDirection::Forward => Vector2::new(MeterPerSec::zero(), -PLAYER_MAX_WALK_SPEED),
        MoveDirection::Backward => Vector2::new(MeterPerSec::zero(), PLAYER_MAX_WALK_SPEED / 2.0),
        MoveDirection::StrafeLeft => Vector2::new(PLAYER_MAX_WALK_SPEED / 2.0, MeterPerSec::zero()),
        MoveDirection::StrafeRight => Vector2::new(-PLAYER_MAX_WALK_SPEED / 2.0, MeterPerSec::zero())
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
