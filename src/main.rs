mod board;
mod size;

use ggez::{GameResult, ContextBuilder, event, Context};
use ggez::event::EventHandler;
use ggez::conf::{WindowSetup, WindowMode, Backend, NumSamples};
use ggez::graphics::*;
use board::Board;
use size::UnitToPixelCalculator;

const BG_COLOR: [f32; 4] = [0.3, 0.3, 0.3, 1.0];
const TILE_SPOT_COLOR: [f32; 4] = [0.2, 0.2, 0.2, 1.0];

const WINDOW_SIZE: (u32, u32) = (510, 510);

struct GameState {
    board: Board,
    unit_to_pixel: UnitToPixelCalculator
}

impl GameState {
    fn new() -> GameState {
        GameState{
            board: Board::new(),
            unit_to_pixel: UnitToPixelCalculator::new(5),
        }
    }

    fn draw_grid(&self, ctx: &mut Context) -> GameResult {
        for tile_x in 0..self.board.get_width() {
            for tile_y in 0..self.board.get_height() {
                let tile_spot = self.board.get_tile_spot((tile_x, tile_y));
                let tile_size = self.unit_to_pixel.to_pixels(&tile_spot.size) as f32;

                let tile_top = tile_y as f32 * (tile_size + 1_f32);

                let tile_left = tile_x as f32 * (tile_size + 1_f32);

                let rect = Rect { x: tile_left, y: tile_top, w: tile_size, h: tile_size };
                let tile_rect = Mesh::new_rectangle(ctx, DrawMode::fill(), rect, TILE_SPOT_COLOR.into())?;

                draw(ctx, &tile_rect, DrawParam::new())?;
            }
        }

        Ok(())
    }
}

impl EventHandler for GameState {
    fn update(&mut self, _ctx: &mut Context) -> GameResult {
        Ok(())
    }

    fn draw(&mut self, ctx: &mut Context) -> GameResult {
        clear(ctx, BG_COLOR.into());
        self.draw_grid(ctx)?;
        present(ctx)?;
        Ok(())
    }
}

pub fn main() -> GameResult {
    let window_mode = WindowMode::default()
        .dimensions(WINDOW_SIZE.0 as f32, WINDOW_SIZE.1 as f32);
    let window_setup = WindowSetup::default()
        .title("Botanique")
        .samples(NumSamples::Zero);
    let context_builder = ContextBuilder::new("Botanique", "surdom")
        .window_setup(window_setup)
        .window_mode(window_mode)
        .backend(Backend::default().version(4, 5));
    let (mut context, mut event_loop) = context_builder.build()?;
    let mut state = GameState::new();
    event::run(&mut context, &mut event_loop, &mut state)
}