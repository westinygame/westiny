use crate::size::*;
use crate::tile::Tile;
use crate::tile::barren_land::BarrenLandTile;
use array_init;
use crate::assets::MappingID;
use ggez::nalgebra::Point2;

pub const BOARD_SIZE : usize = 16;

pub struct Board {
    tiles: [[TileSpot; BOARD_SIZE]; BOARD_SIZE],
    hovered: Option<(usize, usize)>,
}

impl Board {
    pub fn new() -> Board {
        let tiles: [[TileSpot; BOARD_SIZE]; BOARD_SIZE] = array_init::array_init(|_| {
            let col: [TileSpot; BOARD_SIZE] = array_init::array_init(|_| {
                TileSpot::new(Box::new(BarrenLandTile::new()))
            });
            col
        });
        Board {
            tiles,
            hovered: None,
        }
    }

    pub fn get_width(&self) -> usize {
        BOARD_SIZE
    }

    pub fn get_height(&self) -> usize {
        BOARD_SIZE
    }

    pub fn get_tile_spot_by_idx(&mut self, at_pos: (usize, usize)) -> Option<&mut TileSpot> {
        if at_pos.0 > BOARD_SIZE - 1 || at_pos.1 > BOARD_SIZE - 1 {
            None
        } else {
            Some(&mut self.tiles[at_pos.0][at_pos.1])
        }
    }

    pub fn to_tile_idx(&self, pos: Point2<SizeUnit>) -> Option<(usize, usize)> {
        let idx = ((pos.x / TILE_SIZE) as usize, (pos.y / TILE_SIZE) as usize);
        if idx.0 > BOARD_SIZE - 1 || idx.1 > BOARD_SIZE - 1 {
            None
        } else {
            Some(idx)
        }
    }

    pub fn hover(&mut self, hover_idx: (usize, usize)) {
        self.hovered = Some(hover_idx);
    }

    pub fn get_hovered(&self) -> Option<(usize, usize)> {
        self.hovered
    }
}

pub const TILE_SIZE: SizeUnit = 16.0;

pub struct TileSpot {
    pub size: SizeUnit,
    tile: Box<dyn Tile>,
}

impl TileSpot {
    pub fn new(tile: Box<dyn Tile>) -> TileSpot {
        TileSpot {
            size: TILE_SIZE,
            tile,
        }
    }

    pub fn get_texture_id(&self) -> &MappingID {
        self.tile.get_texture_map_id()
    }
}
