pub(crate) use client_registry::ClientID;
pub(crate) use event::{ClientNetworkEvent, NetworkCommand};

pub use client_registry::ClientRegistry;
pub use network_id_supplier::NetworkIdSupplier;
pub use network_stream_id::StreamId;
pub use westiny_common::resources::*;

mod client_registry;
mod event;
mod network_id_supplier;
mod network_stream_id;

pub struct ResourcesDir(pub std::path::PathBuf);
