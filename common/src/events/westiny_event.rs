use crate::network;

#[derive(Clone, Debug, PartialEq)]
pub enum AppEvent {
    Connection(network::Result<network::ClientInitialData>),
    Disconnect,
}

#[derive(Clone, Debug)]
pub enum WestinyEvent {
    /// Window, Ui, Input events
    EngineEvent(StateEvent),
    /// All the application events
    App(AppEvent)
}
