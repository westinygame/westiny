use serde::{Serialize, Deserialize};
use derive_new::new;
use std::fmt::{Display, Debug, Formatter};
use crate::components::{Input, NetworkId};
use amethyst::core::math::Point2;

#[derive(Debug, Serialize, Deserialize)]
pub enum PacketType {
    ConnectionRequest {
        player_name: String
    },
    ConnectionResponse(Result<ClientInitialData>),
    InputState {
        input: Input
    },
    EntityStateUpdate(EntityState),
}

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
pub struct ClientInitialData {
    pub player_network_id: NetworkId,
    pub initial_pos: Point2<f32>,
}

impl Eq for ClientInitialData {}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct EntityState {
    pub network_id: NetworkId,
    pub position: Point2<f32>,
    pub rotation: f32,
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
            ErrorKind::Other => "Other error"
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
