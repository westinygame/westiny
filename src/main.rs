mod board;
mod size;
mod tile;
mod assets;
mod entity;

use std::env;
use std::path;
use ggez::{GameResult, ContextBuilder, event, Context};
use ggez::event::{EventHandler, KeyMods};
use ggez::conf::{WindowSetup, WindowMode, Backend, NumSamples};
use ggez::graphics::*;
use board::{
    Board,
};
use size::UnitToPixelCalculator;
use ggez::nalgebra::{Point2, Vector2};
use assets::TileTexture;
use board::{TILE_SIZE, BOARD_SIZE};
use entity::player::Player;
use std::f32::consts::PI;
use ggez::input::keyboard::KeyCode;
use entity::Direction::*;

const BG_COLOR: [f32; 4] = [0.3, 0.3, 0.3, 1.0];

struct GameState {
    board: Board,
    unit_to_pixel: UnitToPixelCalculator,
    tile_texture: TileTexture,
    player: Player,
    cursor_pos: Point2<f32>,
}

impl GameState {
    fn new(ctx: &mut Context) -> GameState {
        let unit_pixel_calc = UnitToPixelCalculator::new(2);
        let board_middle = (BOARD_SIZE as f32 * TILE_SIZE) / 2.0;
        GameState{
            board: Board::new(),
            unit_to_pixel: unit_pixel_calc,
            tile_texture: TileTexture::create_texture_map(ctx),
            player: Player::new(ctx, Point2::new(board_middle, board_middle)),
            cursor_pos: Point2::origin(),
        }
    }

    fn draw_tiles(&mut self, ctx: &mut Context) -> GameResult {
        let opt_hovered_idx = self.board.get_hovered();
        let tile_size = self.unit_to_pixel.to_pixels(TILE_SIZE) as f32;
        for x in 0..self.board.get_width() {
            for y in 0..self.board.get_height() {
                let tile = self.board.get_tile_spot_by_idx((x, y)).unwrap(); // TODO Error

                let texture_id = tile.get_texture_id();

                let texture = self.tile_texture.get(texture_id)
                    .expect(format!("Texture not found '{}'", texture_id).as_str()); // TODO Error

                let tile_top = y as f32 * (tile_size);
                let tile_left = x as f32 * (tile_size);

                let draw_param = DrawParam::new()
                    .dest(Point2::new(tile_top, tile_left))
                    .scale(Vector2::new(2.0, 2.0));
                draw(ctx, texture, draw_param)?;
            }
        }

        if let Some(hover_idx) = opt_hovered_idx {
            self.draw_border(ctx, hover_idx, tile_size);
        }
        Ok(())
    }

    fn draw_border(&mut self, ctx: &mut Context, coords: (usize, usize), tile_size: f32) {
            let top = coords.0 as f32 * (tile_size);
            let bottom = top + tile_size;
            let left = coords.1 as f32 * (tile_size);
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
        let offset = -1_f32 * self.unit_to_pixel.to_pixels(TILE_SIZE / 2.0) as f32;
        let sin_dir = f32::sin(self.player.get_facing());
        let cos_dir = f32::cos(self.player.get_facing());
        let rotated_offset_x = offset * (cos_dir - sin_dir);
        let rotated_offset_y = offset * (cos_dir + sin_dir);

        let player_pos_x_px = self.unit_to_pixel.to_pixels(self.player.get_position().x) as f32 + rotated_offset_x;
        let player_pos_y_px = self.unit_to_pixel.to_pixels(self.player.get_position().y) as f32 + rotated_offset_y;

        let player_draw_pos = Point2::new(player_pos_x_px, player_pos_y_px);

        let draw_param = DrawParam::default()
            .dest(player_draw_pos)
            .scale(Vector2::new(2.0, 2.0))
            .rotation(self.player.get_facing());
        draw(ctx, self.player.get_texture(), draw_param)
    }
}

impl EventHandler for GameState {
    fn update(&mut self, ctx: &mut Context) -> GameResult {
        const TARGET_FPS: u32 = 60;

        let player_pos_x_px = self.unit_to_pixel.to_pixels(self.player.get_position().x) as f32;
        let player_pos_y_px = self.unit_to_pixel.to_pixels(self.player.get_position().y) as f32;

        let rad_diff = f32::atan((self.cursor_pos.x - player_pos_x_px) / (self.cursor_pos.y - player_pos_y_px));
        let player_facing = if self.cursor_pos.y < player_pos_y_px { -rad_diff + PI } else { -rad_diff };
        self.player.facing(player_facing);

        while ggez::timer::check_update_time(ctx, TARGET_FPS) {
            let sec = 1.0 / TARGET_FPS as f32;

            if let Some(move_realtive_direction) = self.player.get_moving_direction() {
                let move_absolute_direcion = player_facing + move_realtive_direction.rotation();
                let moving_velocity = entity::player::get_moving_velocity(move_realtive_direction);

                let velocity_x = -1.0 * moving_velocity * f32::sin(move_absolute_direcion);
                let velocity_y = moving_velocity * f32::cos(move_absolute_direcion);
                let player_last_pos = self.player.get_position();

                let player_new_pos = Point2::new(player_last_pos.x + velocity_x * sec, player_last_pos.y + velocity_y * sec);
                self.player.position(player_new_pos);
            }
        }

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
        if let Some(tile_idx) = self.board.to_tile_idx(Point2::new(x_unit, y_unit)) {
            self.board.hover(tile_idx);
        }
        self.cursor_pos = Point2::new(x, y);
    }

    fn key_down_event(&mut self, _ctx: &mut Context, keycode: KeyCode, _keymods: KeyMods, repeat: bool) {
        if repeat == false {
            match keycode {
                KeyCode::W => self.player.move_direction(FORWARD),
                KeyCode::A => self.player.move_direction(LEFT),
                KeyCode::S => self.player.move_direction(BACKWARD),
                KeyCode::D => self.player.move_direction(RIGHT),
                _ => {}
            }
        }
    }

    fn key_up_event(&mut self, _ctx: &mut Context, keycode: KeyCode, _keymods: KeyMods) {
        if let KeyCode::W | KeyCode::A | KeyCode::S | KeyCode::D = keycode {
            self.player.stop_move();
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
        .title("Westiny")
        .samples(NumSamples::Zero);
    let context_builder = ContextBuilder::new("Westiny", "surdom")
        .window_setup(window_setup)
        .window_mode(window_mode)
        .backend(Backend::default().version(3, 2))
        .add_resource_path(resource_dir);
    let (mut context, mut event_loop) = context_builder.build()?;

    set_default_filter(&mut context, FilterMode::Nearest);
    let mut state = GameState::new(&mut context);
    event::run(&mut context, &mut event_loop, &mut state)
}

//#[derive(Copy, Clone)]
//pub struct Point<T> where T: Sized {
//    pub x: T,
//    pub y: T,
//}
//
//impl<T> Point<T> {
//    pub fn new(x: T, y: T) -> Self {
//        Point {x, y}
//    }
//}
//
//impl<T> Into<Point2<T>> for Point<T> where T: Copy + PartialEq + std::fmt::Debug + 'static {
//    fn into(self) -> Point2<T> {
//        Point2::new(self.x, self.y)
//    }
//}
//
//impl<T> From<Point2<T>> for Point<T> where T:  Copy + PartialEq + std::fmt::Debug + 'static {
//    fn from(p: Point2<T>) -> Self {
//        unsafe {
//            Point {
//                x: *p.get_unchecked(0),
//                y: *p.get_unchecked(1),
//            }
//        }
//    }
//}
