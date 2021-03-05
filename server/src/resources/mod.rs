pub(crate) use client_registry::ClientID;
pub(crate) use event::{ClientNetworkEvent, NetworkCommand};
pub(crate) use network_stream_id::StreamId;
pub(crate) use damage::DamageEvent;

pub use network_id_supplier::NetworkIdSupplier;
pub use client_registry::ClientRegistry;

mod client_registry;
mod event;
mod network_id_supplier;
mod network_stream_id;
pub mod collision;
mod damage;
