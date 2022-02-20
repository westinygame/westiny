use crate::components::{BoundingCircle, Health, Input, NetworkId, Player, SpriteId, WeaponInfo};
use bevy::prelude::{BuildChildren, Bundle, Commands, Entity, Transform};
use westiny_common::entities::SimpleSpriteSheetBundle;
use westiny_common::metric_dimension::length::Meter;

pub const CHARACTER_HEIGHT: f32 = 1.8;

#[derive(Bundle)]
pub struct PlayerCharacterBundle {
    pub net_id: NetworkId,
    pub bounding_circle: BoundingCircle,

    #[bundle]
    pub sprite_sheet_bundle: SimpleSpriteSheetBundle,
}

impl PlayerCharacterBundle {
    pub fn new(net_id: NetworkId, mut transform: Transform) -> Self {
        transform.translation.z = CHARACTER_HEIGHT;
        Self {
            net_id,
            bounding_circle: BoundingCircle { radius: Meter(0.5) },
            sprite_sheet_bundle: SimpleSpriteSheetBundle::new(transform, SpriteId::Player),
        }
    }
}

#[derive(Bundle)]
pub struct ThisPlayerBundle {
    pub player: Player,
    pub health: Health,
    pub input: Input,
    pub weapon_info: WeaponInfo,
}

impl ThisPlayerBundle {
    pub fn new() -> Self {
        ThisPlayerBundle {
            player: Player,
            health: Health(100),
            input: Input::default(),
            weapon_info: WeaponInfo {
                magazine_size: 6,
                bullets_in_magazine: 6,
                name: "Revolver".to_string(),
            },
        }
    }
}

pub fn create_player_character(
    commands: &mut Commands,
    net_id: NetworkId,
    transform: Transform,
) -> Entity {
    commands
        .spawn_bundle(PlayerCharacterBundle::new(net_id, transform))
        .with_children(|parent| {
            // hand with pistol
            parent.spawn_bundle(SimpleSpriteSheetBundle::new(
                Transform::from_xyz(Meter(-0.25).into_pixel(), Meter(-0.2).into_pixel(), -0.3), // relative to parent
                SpriteId::HandWithPistol,
            ));
        })
        .id()
}

pub fn create_this_player(commands: &mut Commands, net_id: NetworkId, transform: Transform) {
    let entity = create_player_character(commands, net_id, transform);
    commands
        .entity(entity)
        .insert_bundle(ThisPlayerBundle::new());
}
