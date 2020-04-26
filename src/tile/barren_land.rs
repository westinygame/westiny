use crate::tile::Tile;
use crate::assets::MappingID;

pub struct BarrenLandTile {}

impl BarrenLandTile {
    pub const TEXTURE_MAPPING_ID: MappingID = MappingID { module: "base", entity: "barren-land"};
    pub fn new() -> Self {
        BarrenLandTile {}
    }
}

impl Tile for BarrenLandTile {
    fn get_texture_map_id(&self) -> &MappingID {
        &BarrenLandTile::TEXTURE_MAPPING_ID
    }
}

