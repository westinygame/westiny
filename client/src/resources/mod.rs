pub use audio::{initialize_audio, Sounds};
pub use hud::{format_health, Hud, initialize_hud};
pub use sprite_resource::{initialize_sprite_resource, SpriteResource};
pub use network_stream_id::StreamId;
use westiny_common::components::NetworkId;

mod audio;
mod hud;
mod sprite_resource;
mod network_stream_id;

pub struct PlayerNetworkId(pub NetworkId);