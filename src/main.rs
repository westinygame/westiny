mod board;
mod size;

use ggez::{GameResult, ContextBuilder, event, Context};
use ggez::event::EventHandler;
use ggez::conf::{WindowSetup, WindowMode, Backend, NumSamples};
use ggez::graphics::*;
use board::Board;
use size::UnitToPixelCalculator;
use ggez::nalgebra::Point2;

const BG_COLOR: [f32; 4] = [0.3, 0.3, 0.3, 1.0];

struct GameState {
    board: Board,
    unit_to_pixel: UnitToPixelCalculator,
}

impl GameState {
    fn new() -> GameState {
        GameState{
            board: Board::new(),
            unit_to_pixel: UnitToPixelCalculator::new(2),
        }
    }

    fn draw_tiles(&mut self, ctx: &mut Context) -> GameResult {
        let mut mb = MeshBuilder::new();
        for x in 0..self.board.get_width() {
            for y in 0..self.board.get_height() {
                self.refresh_tile( Point::new(x, y), &mut mb)?;
            }
        }
        let mesh = &mb.build(ctx)?;
        draw(ctx, mesh, DrawParam::new())?;
        Ok(())
    }

    fn refresh_tile(&mut self, idx: Point<usize>, mb: &mut MeshBuilder) -> GameResult {
        let tile = self.board.get_tile_spot_by_idx(idx).unwrap();
        let tile_color = tile.get_view().get_color();
        let tile_size = self.unit_to_pixel.to_pixels(&tile.size) as f32;

        let tile_top = idx.y as f32 * (tile_size);
        let tile_left = idx.x as f32 * (tile_size);

        let rect = Rect { x: tile_left, y: tile_top, w: tile_size, h: tile_size};
        mb.rectangle(DrawMode::fill(), rect, tile_color);
        Ok(())
    }
}

impl EventHandler for GameState {
    fn update(&mut self, _ctx: &mut Context) -> GameResult {
        Ok(())
    }

    fn draw(&mut self, ctx: &mut Context) -> GameResult {
        clear(ctx, BG_COLOR.into());
        self.draw_tiles(ctx)?;
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
    let window_mode = WindowMode::default()
        .dimensions(1024_f32, 1024_f32); // TODO get resolution from config
//        .fullscreen_type(FullscreenType::Desktop);
    let window_setup = WindowSetup::default()
        .title("Botanique")
        .samples(NumSamples::Two);
    let context_builder = ContextBuilder::new("Botanique", "surdom")
        .window_setup(window_setup)
        .window_mode(window_mode)
        .backend(Backend::default().version(3, 2));
    let (mut context, mut event_loop) = context_builder.build()?;


    let mut state = GameState::new();
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
