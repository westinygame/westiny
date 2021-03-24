use serde::{Serialize, Deserialize};
use derive_new::new;
use std::fmt::{Display, Debug, Formatter};
use crate::components::{Input, NetworkId, Health};
use amethyst::core::math::{Point2, Vector2};
use crate::resources::Seed;
use crate::PlayerName;

#[derive(Debug, Serialize, Deserialize)]
#[cfg_attr(test, derive(Clone, PartialEq))]
pub enum PacketType {
    ConnectionRequest {
        player_name: String
    },
    ConnectionResponse(Result<ClientInitialData>),
    InputState {
        input: Input
    },
    EntityStateUpdate(Vec<EntityState>),
    EntityDelete(NetworkEntityDelete),
    PlayerUpdate(PlayerUpdate),
    Notification(PlayerNotification),
    ShotEvent(ShotEvent),
    PlayerDeath(PlayerDeath),
}

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
pub struct ClientInitialData {
    pub player_network_id: NetworkId,
    pub initial_pos: Point2<f32>,
    pub seed: Seed,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
#[cfg_attr(test, derive(PartialEq))]
pub struct EntityState {
    pub network_id: NetworkId,
    pub position: Point2<f32>,
    pub rotation: f32,
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
    pub position: Point2<f32>,
    pub velocity: Vector2<f32>,
    pub bullet_time_limit_secs: f32,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
#[cfg_attr(test, derive(PartialEq))]
pub struct PlayerDeath {
    pub player_name: PlayerName,
    pub position: Point2<f32>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
#[cfg_attr(test, derive(PartialEq))]
pub enum PlayerUpdate {
    HealthUpdate(Health),
    AmmoUpdate {
        ammo_in_magazine: u32,
    }
}

#[derive(Copy, Clone, Serialize, Deserialize, Debug, PartialEq)]
pub enum ErrorKind {
    AlreadyConnected,
    Other,
}

impl Display for ErrorKind {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let literal = match self {
            ErrorKind::AlreadyConnected => "Client already connected",
            ErrorKind::Other => "Other error"
        };

        write!(f, "{}", literal)
    }
}

#[derive(Copy, Clone, Debug, Serialize, Deserialize, new, PartialEq)]
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
