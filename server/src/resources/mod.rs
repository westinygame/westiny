pub(crate) use client_registry::ClientID;
pub(crate) use event::ClientNetworkEvent;

pub use client_registry::ClientRegistry;

mod client_registry;
mod event;
