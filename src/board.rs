use crate::size::*;
use ggez::graphics::Color;
use crate::Point;

pub const BOARD_SIZE : usize = 32;

pub struct Board {
    tiles: [[TileSpot; BOARD_SIZE]; BOARD_SIZE],
    hovered: Option<Point<usize>>,
}

impl Board {
    pub fn new() -> Board {
        Board {
            tiles: [[TileSpot::new(); BOARD_SIZE]; BOARD_SIZE],
            hovered: None,
        }
    }

    pub fn get_width(&self) -> usize {
        BOARD_SIZE
    }

    pub fn get_height(&self) -> usize {
        BOARD_SIZE
    }

    pub fn get_tile_spot_by_idx(&mut self, at_pos: Point<usize>) -> Option<&mut TileSpot> {
        if at_pos.x > BOARD_SIZE - 1 || at_pos.y > BOARD_SIZE - 1 {
            None
        } else {
            Some(&mut self.tiles[at_pos.x][at_pos.y])
        }
    }

    pub fn to_tile_idx(&self, pos: Point<SizeUnit>) -> Option<Point<usize>> {
        let idx = Point::new((pos.x / TILE_SIZE) as usize, (pos.y / TILE_SIZE) as usize);
        if idx.x > BOARD_SIZE - 1 || idx.y > BOARD_SIZE - 1 {
            None
        } else {
            Some(idx)
        }
    }

    pub fn hover(&mut self, hover_idx: Point<usize>) -> Option<Point<usize>> {
        let old_hover_idx= self.hovered;
        if let Some(i) = &self.hovered {
            let old_hover = &mut self.tiles[i.x][i.y];
            old_hover.view.unhover();
        }

        let new_hover = &mut self.tiles[hover_idx.x][hover_idx.y];
        self.hovered = Some(hover_idx);
        new_hover.view.hover();
        old_hover_idx
    }
}

pub const TILE_SIZE: SizeUnit = 16;

#[derive(Copy, Clone)]
pub struct TileSpot {
    pub size: SizeUnit,
    view: TileView,
}

impl TileSpot {
    pub fn new() -> TileSpot {
        TileSpot {
            size: TILE_SIZE,
            view: TileView::new(),
        }
    }

    pub fn get_view(&mut self) -> &mut TileView {
        &mut self.view
    }
}

#[derive(Copy, Clone)]
pub struct TileView {
    color: Color,
}

const DEFAULT_TILE_COLOR: [f32; 4] = [0.2, 0.2, 0.2, 1.0];
const HOVER_TILE_COLOR:[f32; 4] = [0.6, 0.2, 0.2, 1.0];

impl TileView {
    pub fn new() -> TileView {
        TileView {
            color: DEFAULT_TILE_COLOR.into(),
        }
    }

    pub fn get_color(&self) -> Color {
        self.color
    }

    fn set_color(&mut self, color: Color) {
        self.color = color;
    }

    fn hover(&mut self) {
        self.set_color(HOVER_TILE_COLOR.into());
    }
    fn unhover(&mut self) {
        self.set_color(DEFAULT_TILE_COLOR.into());
    }
}
