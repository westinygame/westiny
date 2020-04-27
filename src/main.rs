mod board;
mod size;
mod tile;
mod assets;
mod player;

use std::env;
use std::path;
use ggez::{GameResult, ContextBuilder, event, Context};
use ggez::event::EventHandler;
use ggez::conf::{
    WindowSetup,
    WindowMode,
    Backend,
};
use ggez::graphics::*;
use board::{
    Board,
};
use size::UnitToPixelCalculator;
use ggez::nalgebra::{Point2, Vector2};
use assets::TileTexture;
use crate::board::TILE_SIZE;
use crate::player::Player;
use crate::size::SizeUnit;

const BG_COLOR: [f32; 4] = [0.3, 0.3, 0.3, 1.0];

struct GameState {
    board: Board,
    unit_to_pixel: UnitToPixelCalculator,
    tile_texture: TileTexture,
    player: Player,
}

impl GameState {
    fn new(ctx: &mut Context) -> GameState {
        GameState{
            board: Board::new(),
            unit_to_pixel: UnitToPixelCalculator::new(2),
            tile_texture: TileTexture::create_texture_map(ctx),
            player: Player::new(ctx, Point::new(256_f32, 256_f32)),
        }
    }

    fn draw_tiles(&mut self, ctx: &mut Context) -> GameResult {
        let opt_hovered_idx = self.board.get_hovered();
        let tile_size = self.unit_to_pixel.to_pixels(&TILE_SIZE) as f32;
        for x in 0..self.board.get_width() {
            for y in 0..self.board.get_height() {
                let idx = Point::new(x, y);
                let tile = self.board.get_tile_spot_by_idx(idx).unwrap(); // TODO Error

                let texture_id = tile.get_texture_id();

                let texture = self.tile_texture.get(texture_id)
                    .expect(format!("Texture not found '{}'", texture_id).as_str()); // TODO Error

                let tile_top = idx.y as f32 * (tile_size);
                let tile_left = idx.x as f32 * (tile_size);

                let draw_param = DrawParam::new().dest(Point2::new(tile_top, tile_left)).scale(Vector2::new(2.0, 2.0));
                draw(ctx, texture, draw_param)?;
            }
        }

        if let Some(hover_idx) = opt_hovered_idx {
            self.draw_border(ctx, hover_idx, tile_size);
        }
        Ok(())
    }

    fn draw_border(&mut self, ctx: &mut Context, coords: Point<usize>, tile_size: f32) {
            let top = coords.x as f32 * (tile_size);
            let bottom = top + tile_size;
            let left = coords.y as f32 * (tile_size);
            let right = left + tile_size;

            let top_left = Point2::new(top, left);
            let top_right = Point2::new(top, right);
            let bottom_left = Point2::new(bottom, left);
            let bottom_right = Point2::new(bottom, right);

            let mesh = &MeshBuilder::default()   // TODO Error
                .line(&[top_left, top_right], 2_f32, BLACK).unwrap()
                .line(&[bottom_left, bottom_right], 2_f32, BLACK).unwrap()
                .line(&[top_left, bottom_left], 2_f32, BLACK).unwrap()
                .line(&[top_right, bottom_right], 2_f32, BLACK).unwrap()
                .build(ctx).unwrap();

            draw(ctx, mesh, DrawParam::new()).unwrap_or_default(); // TODO handle Result
    }

    fn draw_player(&mut self, ctx: &mut Context) -> GameResult {
        let offset = -1_f32 * self.unit_to_pixel.to_pixels(&(TILE_SIZE/2)) as f32;
        let player_draw_pos = Point2::new(self.player.get_position().x + offset, self.player.get_position().y + offset);
        let draw_param = DrawParam::default()
            .dest(player_draw_pos)
            .scale(Vector2::new(2.0, 2.0));
        draw(ctx, self.player.get_texture(), draw_param)
    }
}

impl EventHandler for GameState {
    fn update(&mut self, _ctx: &mut Context) -> GameResult {
        Ok(())
    }

    fn draw(&mut self, ctx: &mut Context) -> GameResult {
        clear(ctx, BG_COLOR.into());
        self.draw_tiles(ctx)?;
        self.draw_player(ctx)?;
        present(ctx)?;
        Ok(())
    }

    fn mouse_motion_event(&mut self, _ctx: &mut Context, x: f32, y: f32, _dx: f32, _dy: f32) {
        let x_unit = self.unit_to_pixel.to_units(x as u32);
        let y_unit = self.unit_to_pixel.to_units(y as u32);
        if let Some(tile_idx) = self.board.to_tile_idx(Point::new(x_unit, y_unit)) {
            self.board.hover(tile_idx);
        }
    }

}

pub fn main() -> GameResult {

    let resource_dir = if let Ok(manifest_dir) = env::var("CARGO_MANIFEST_DIR") {
        let mut path = path::PathBuf::from(manifest_dir);
        path.push("resources");
        path
    } else {
        path::PathBuf::from("./resources")
    };

    let window_mode = WindowMode::default()
        .dimensions(512_f32, 512_f32); // TODO get resolution from config
//        .fullscreen_type(FullscreenType::Desktop);
    let window_setup = WindowSetup::default()
        .title("Botanique");
    let context_builder = ContextBuilder::new("Botanique", "surdom")
        .window_setup(window_setup)
        .window_mode(window_mode)
        .backend(Backend::default().version(3, 2))
        .add_resource_path(resource_dir);
    let (mut context, mut event_loop) = context_builder.build()?;

    set_default_filter(&mut context, FilterMode::Nearest);
    let mut state = GameState::new(&mut context);
    event::run(&mut context, &mut event_loop, &mut state)
}

#[derive(Copy, Clone)]
pub struct Point<T> where T: Sized {
    pub x: T,
    pub y: T,
}

impl<T> Point<T> {
    pub fn new(x: T, y: T) -> Self {
        Point {x, y}
    }
}

impl<T> Into<Point2<T>> for Point<T> where T: Copy + PartialEq + std::fmt::Debug + 'static {
    fn into(self) -> Point2<T> {
        Point2::new(self.x, self.y)
    }
}

impl<T> From<Point2<T>> for Point<T> where T:  Copy + PartialEq + std::fmt::Debug + 'static {
    fn from(p: Point2<T>) -> Self {
        unsafe {
            Point {
                x: *p.get_unchecked(0),
                y: *p.get_unchecked(1),
            }
        }
    }
}
