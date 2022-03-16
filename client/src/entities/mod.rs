pub use player::{create_player_character, create_this_player};
pub use tilemap::initialize_tilemap;
pub use westiny_common::entities::*;

mod player;
pub mod tilemap;

use crate::components::{Lifespan, SpriteId};
use bevy::prelude::{Bundle, Transform};
use westiny_common::metric_dimension::Second;

#[derive(Bundle)]
pub struct CorpseBundle {
    pub lifespan: Lifespan,

    #[bundle]
    pub simple_spritesheet: SimpleSpriteSheetBundle,
}

const CORPSE_HEIGHT: f32 = 0.1;

impl CorpseBundle {
    pub fn new(mut transform: Transform, current_time: std::time::Duration) -> Self {
        transform.translation.z = CORPSE_HEIGHT;
        Self {
            lifespan: Lifespan::new(Second(60.0), current_time),
            simple_spritesheet: SimpleSpriteSheetBundle::new(transform, SpriteId::Corpse),
        }
    }
}

