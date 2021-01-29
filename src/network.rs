use amethyst::core::Transform;
use serde::{Serialize, Deserialize};
use derive_new::new;
use std::fmt::{Display, Debug};
use serde::__private::Formatter;

#[derive(Clone, Serialize, Deserialize)]
pub enum PackageType {
    // player name
    ConnectionRequest(String),
}

#[derive(Serialize, Deserialize, new)]
pub struct ConnectionPackage {
    pub initial_trans: Transform,
}

#[derive(Copy, Clone, Serialize, Deserialize, Debug)]
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

#[derive(Debug, Serialize, Deserialize, new)]
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