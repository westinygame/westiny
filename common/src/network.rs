use serde::{Serialize, Deserialize};
use derive_new::new;
use std::fmt::{Display, Debug, Formatter};
use crate::components::{Input, NetworkId};
use amethyst::core::math::Point2;
use crate::resources::Seed;

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
    EntityStateUpdate(EntityState),
    EntityDelete(NetworkEntityDelete),
    EntityHealthUpdate(EntityHealth),
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
pub struct NetworkEntityDelete
{
    pub network_id: NetworkId,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
#[cfg_attr(test, derive(PartialEq))]
pub struct EntityHealth
{
    pub network_id: NetworkId,
    pub health: u16,
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
