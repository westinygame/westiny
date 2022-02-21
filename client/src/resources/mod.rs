//pub use audio::{initialize_audio, Sounds};
//pub use hud::{format_health, format_ammo, Hud, initialize_hud};
pub use network_stream_id::StreamId;
//pub use notification_bar::{NotificationBar};
pub use sprite_resource::{initialize_sprite_resource, SpriteResource};
pub use westiny_common::resources::*;

use westiny_common::components::{EntityType, NetworkId};

//mod audio;
//mod hud;
//mod notification_bar;
mod network_stream_id;
mod sprite_resource;

pub struct PlayerNetworkId(pub NetworkId);

impl std::default::Default for PlayerNetworkId {
    fn default() -> Self {
        PlayerNetworkId(NetworkId {
            entity_type: EntityType::Player,
            id: u32::MAX,
        })
    }
}
