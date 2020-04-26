use crate::size::*;
use crate::Point;
use crate::tile::Tile;
use crate::tile::barren_land::BarrenLandTile;
use array_init;
use crate::assets::MappingID;

pub const BOARD_SIZE : usize = 16;

pub struct Board {
    tiles: [[TileSpot; BOARD_SIZE]; BOARD_SIZE],
    hovered: Option<Point<usize>>,
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
        self.hovered = Some(hover_idx);
        old_hover_idx
    }

    pub fn get_hovered(&self) -> Option<Point<usize>> {
        self.hovered
    }
}

pub const TILE_SIZE: SizeUnit = 16;

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
