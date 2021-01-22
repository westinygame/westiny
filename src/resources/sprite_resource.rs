
use amethyst::prelude::*;
use amethyst::renderer::{ImageFormat, SpriteSheet, SpriteSheetFormat, SpriteRender, Texture};
use amethyst::assets::{Handle, Loader, AssetStorage};

#[derive(Clone)]
pub struct SpriteResource {
    pub sheet: Handle<SpriteSheet>
}

impl SpriteResource {
    pub fn sprite_render_for_player(&self) -> SpriteRender {
        self.sprite_render_at_index(3)
    }

    pub fn sprite_render_for_shooting_player(&self) -> SpriteRender {
        self.sprite_render_at_index(4)
    }

    pub fn sprite_render_for_bullet(&self) -> SpriteRender {
        self.sprite_render_at_index(5)
    }

    fn sprite_render_at_index(&self, index: usize) -> SpriteRender {
        SpriteRender {
            sprite_sheet: self.sheet.clone(),
            sprite_number: index
        }
    }
}

pub fn initialize_sprite_resource(world: &mut World) -> SpriteResource
{
    let res = SpriteResource { sheet: load_sprite_sheet(world) };
    world.insert(res.clone());
    res
}

fn load_sprite_sheet(world: &mut World) -> Handle<SpriteSheet> {
    let texture_handle = {
        let loader = world.read_resource::<Loader>();
        let texture_storage = world.read_resource::<AssetStorage<Texture>>();
        loader.load(
            "spritesheet.png",
            ImageFormat::default(),
            (),
            &texture_storage,
        )
    };
    let loader = world.read_resource::<Loader>();
    let sprite_sheet_store = world.read_resource::<AssetStorage<SpriteSheet>>();
    loader.load(
        "spritesheet.ron",
        SpriteSheetFormat(texture_handle),
        (),
        &sprite_sheet_store,
    )
}