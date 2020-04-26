use ggez::graphics::Image;
use std::collections::HashMap;
use ggez::Context;
use crate::tile::barren_land;
use std::fmt::{Display, Formatter};
use std::fmt;
use std::path::Path;

#[derive(Eq, PartialEq, Hash, Copy, Clone)]
pub struct MappingID {
    pub module: &'static str,
    pub entity: &'static str,
}

impl Display for MappingID {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "{}:{}", self.module, self.entity)
    }
}

pub const DEFAULT_TEXTURE_ID: MappingID = MappingID { module: "base", entity: "default" };

pub struct TileTexture {
    map: HashMap<MappingID, Image>,
}

impl TileTexture {
    pub fn create_texture_map(ctx: &mut Context) -> TileTexture {
        let mut map = HashMap::new();

        //  put tile texture regisrations here
        TileTexture::register(ctx, DEFAULT_TEXTURE_ID, Path::new("/default.png"), &mut map);
        TileTexture::register(ctx, barren_land::BarrenLandTile::TEXTURE_MAPPING_ID, Path::new("/barren.png"), &mut map);

        TileTexture { map }
    }

    fn register(ctx: &mut Context, texture_id: MappingID, img_path: &Path, map: &mut HashMap<MappingID, Image>) {
        let img = Image::new(ctx, img_path.as_os_str())
            .expect(format!("Unable to load texture with ID {}", texture_id).as_str());
        map.insert(texture_id, img);
    }

    pub fn get(&self, id: &MappingID) -> Option<&Image> {
        self.map.get(id)
    }
}