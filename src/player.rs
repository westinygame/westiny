use crate::Point;
use crate::assets::MappingID;
use ggez::graphics::Image;
use ggez::Context;
use std::f32::consts::{FRAC_2_PI, FRAC_1_PI, PI};

pub struct Player {
    /// The position of the player's central point
    /// TODO Use size units instead of pixels
    pos: Point<f32>,
    texture: Image,

    /// direction in radians. Tells the deviation from zero (positive Y).
    direction: f32,
}

impl Player {
    pub fn new(ctx: &mut Context, initial_pos: Point<f32>) -> Player {
        Player {
            pos: initial_pos,
            texture: Image::new(ctx, "/player.png").unwrap(), // TODO Error
            direction: 0_f32
        }
    }

    pub fn get_texture(&self) -> &Image {
        &self.texture
    }

    pub fn get_position(&self) -> &Point<f32> {
        &self.pos
    }

    pub fn get_direction(&self) -> f32 {
        self.direction
    }

    pub fn direction(&mut self, new_direction: f32) {
        self.direction = new_direction;
    }
}

