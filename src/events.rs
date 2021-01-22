use std::net::SocketAddr;
use amethyst::StateEvent;

use amethyst::core::{
    EventReader,
    ecs::{
        Read,
        SystemData,
        World
    },
    shrev::{
        ReaderId,
        EventChannel
    },
};
use amethyst::derive::EventReader;

#[derive(Clone, Debug)]
pub enum AppEvent {
    Connected(SocketAddr),
}

#[derive(Clone, Debug, EventReader)]
#[reader(WestinyEventReader)]
pub enum WestinyEvent {
    /// Window, Ui, Input events
    EngineEvent(StateEvent),
    /// All the application events
    App(AppEvent)
}