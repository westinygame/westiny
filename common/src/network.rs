use crate::components::{Health, Input, NetworkId};
use crate::metric_dimension::{length::MeterVec2, MeterPerSecVec2, Second};
use crate::resources::Seed;
use crate::PlayerName;
use derive_new::new;
use serde::{Deserialize, Serialize};
use std::fmt::{Debug, Display, Formatter};

#[derive(Debug, Serialize, Deserialize)]
#[cfg_attr(test, derive(Clone, PartialEq))]
pub enum PacketType {
    ConnectionRequest { player_name: String },
    ConnectionResponse(Result<ClientInitialData>),
    InputState { input: Input },
    EntityStateUpdate(Vec<EntityState>),
    EntityDelete(NetworkEntityDelete),
    PlayerUpdate(PlayerUpdate),
    Notification(PlayerNotification),
    ShotEvent(ShotEvent),
    PlayerDeath(PlayerDeath),
}

#[derive(Clone, Debug, Serialize, Deserialize, Eq, PartialEq, Hash)]
pub struct ClientInitialData {
    pub player_network_id: NetworkId,
    pub seed: Seed,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
#[cfg_attr(test, derive(PartialEq))]
pub struct EntityState {
    pub network_id: NetworkId,
    pub position: MeterVec2,
    pub angle: f32,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
#[cfg_attr(test, derive(PartialEq))]
pub struct NetworkEntityDelete {
    pub network_id: NetworkId,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
#[cfg_attr(test, derive(PartialEq))]
pub struct PlayerNotification {
    pub message: String,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
#[cfg_attr(test, derive(PartialEq))]
pub struct ShotEvent {
    pub position: MeterVec2,
    pub velocity: MeterPerSecVec2,
    pub bullet_time_limit_secs: Second,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
#[cfg_attr(test, derive(PartialEq))]
pub struct PlayerDeath {
    pub player_name: PlayerName,
    pub position: MeterVec2,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
#[cfg_attr(test, derive(PartialEq))]
pub enum PlayerUpdate {
    HealthUpdate(Health),
    AmmoUpdate {
        ammo_in_magazine: u32,
    },
    WeaponSwitch {
        name: String,
        magazine_size: u32,
        ammo_in_magazine: u32,
    },
}

#[derive(Copy, Clone, Serialize, Deserialize, Debug, Eq, PartialEq)]
pub enum ErrorKind {
    AlreadyConnected,
    Other,
}

impl Display for ErrorKind {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let literal = match self {
            ErrorKind::AlreadyConnected => "Client already connected",
            ErrorKind::Other => "Other error",
        };

        write!(f, "{}", literal)
    }
}

#[derive(Copy, Clone, Debug, Serialize, Deserialize, new, Eq, PartialEq)]
pub struct Error {
    error_kind: ErrorKind,
}

impl Display for Error {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "Westiny network error: {}", self.error_kind)
    }
}

impl std::error::Error for Error {}

pub type Result<T> = std::result::Result<T, Error>;

