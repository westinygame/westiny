use crate::size::*;

const BOARD_SIZE : usize = 10;
const TILE_SIZE: SizeUnit = 10;

pub(crate) struct Board {
    tiles: [[TileSpot; BOARD_SIZE]; BOARD_SIZE],
}

impl Board {
    pub fn new() -> Board {
        Board {
            tiles: [[TileSpot::new(); BOARD_SIZE]; BOARD_SIZE]
        }
    }

    pub fn get_width(&self) -> usize {
        BOARD_SIZE
    }

    pub fn get_height(&self) -> usize {
        BOARD_SIZE
    }

    pub fn get_tile_spot(&self, at_pos: (usize, usize)) -> &TileSpot {
        &self.tiles[at_pos.0][at_pos.1]
    }
}

#[derive(Copy, Clone)]
pub struct TileSpot {
    pub size: SizeUnit,
}

impl TileSpot {
    pub fn new() -> TileSpot {
        TileSpot {
            size: TILE_SIZE,
        }
    }
}
