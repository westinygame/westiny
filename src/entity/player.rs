use ggez::graphics::Image;
use ggez::Context;
use crate::board::TILE_SIZE;
use crate::size::SizeUnit;
use super::Direction;
use ggez::nalgebra::Point2;

pub struct Player {
    /// The position of the player's central point
    pos: Point2<SizeUnit>,
    texture: Image,

    /// facing direction in radians. Tells the deviation from zero (positive Y).
    facing: f32,

    moving_direction: Option<Direction>,
}

/// The absolute value of the walking player's velocity.
/// Vector might be calculated from its direction.
pub const WALKING_VELOCITY: SizeUnit = TILE_SIZE * 4.0;

pub fn get_moving_velocity(direction: Direction) -> SizeUnit {
    if let Direction::FORWARD = direction {
        WALKING_VELOCITY
    } else {
        WALKING_VELOCITY / 2.0
    }
}

impl Player {
    pub fn new(ctx: &mut Context, initial_pos: Point2<SizeUnit>) -> Player {
        Player {
            pos: initial_pos,
            texture: Image::new(ctx, "/player.png").unwrap(), // TODO Error
            facing: 0.0,
            moving_direction: None,
        }
    }

    pub fn get_texture(&self) -> &Image {
        &self.texture
    }

    pub fn get_position(&self) -> &Point2<SizeUnit> {
        &self.pos
    }

    pub fn position(&mut self, new: Point2<SizeUnit>) {
        self.pos = new;
    }

    pub fn get_facing(&self) -> f32 {
        self.facing
    }

    pub fn facing(&mut self, new_direction: f32) {
        self.facing = new_direction;
    }

    pub fn get_moving_direction(&self) -> Option<Direction> {
        self.moving_direction
    }

    pub fn move_direction(&mut self, direction: Direction) {
        self.moving_direction = Some(direction);
    }

    pub fn stop_move(&mut self) {
        self.moving_direction = None;
    }
}

