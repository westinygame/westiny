pub mod barren_land;
use crate::assets::MappingID;

pub trait Tile {
    fn get_texture_map_id(&self) -> &MappingID;
}