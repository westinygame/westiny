//pub use audio::{initialize_audio, Sounds};
pub use network_stream_id::StreamId;
pub use sprite_resource::{initialize_sprite_resource, SpriteResource};
pub use westiny_common::resources::*;

use westiny_common::components::{EntityType, NetworkId};

//mod audio;
mod network_stream_id;
mod sprite_resource;

#[derive(Debug)]
#[cfg_attr(test, derive(Eq, PartialEq))]
pub struct PlayerNetworkId(pub NetworkId);

impl std::default::Default for PlayerNetworkId {
    fn default() -> Self {
        PlayerNetworkId(NetworkId {
            entity_type: EntityType::Player,
            id: u32::MAX,
        })
    }
}
