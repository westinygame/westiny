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
use westiny_common::network;

#[derive(Clone, Debug)]
#[cfg_attr(test, derive(PartialEq))]
pub enum AppEvent {
    Connection(network::Result<network::ClientInitialData>),
}

#[derive(Clone, Debug, EventReader)]
#[reader(WestinyEventReader)]
pub enum WestinyEvent {
    /// Window, Ui, Input events
    EngineEvent(StateEvent),
    /// All the application events
    App(AppEvent)
}