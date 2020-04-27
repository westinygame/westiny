use crate::Point;
use crate::assets::MappingID;
use ggez::graphics::Image;
use ggez::Context;

pub struct Player {
    /// The position of the player's central point
    /// TODO Use size units instead of pixels
    pos: Point<f32>,
    texture: Image,
}

impl Player {
    pub fn new(ctx: &mut Context, initial_pos: Point<f32>) -> Player {
        Player {
            pos: initial_pos,
            texture: Image::new(ctx, "/hat.png").unwrap(),
        }
    }

    pub fn get_texture(&self) -> &Image {
        &self.texture
    }

    pub fn get_position(&self) -> &Point<f32> {
        &self.pos
    }
}

