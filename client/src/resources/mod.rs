pub use audio::{initialize_audio, Sounds};
pub use hud::{format_health, Hud, initialize_hud};
pub use network_stream_id::StreamId;
pub use sprite_resource::{initialize_sprite_resource, SpriteResource};
pub use groundtile::GroundTile;
use westiny_common::components::NetworkId;

mod audio;
mod hud;
mod sprite_resource;
mod network_stream_id;
mod groundtile;

pub struct PlayerNetworkId(pub NetworkId);
