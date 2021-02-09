pub mod systems;
pub mod resources;

use std::fmt;
use ::serde::{Serialize, Deserialize};
use amethyst::input::BindingTypes;

use westiny_common::MoveDirection;

#[derive(Clone, Debug, Hash, PartialEq, Eq, Serialize, Deserialize)]
pub enum ActionBinding {
    Forward,
    Backward,
    StrafeLeft,
    StrafeRight,
    Shoot,
    Use,
}

#[derive(Clone, Debug, Hash, PartialEq, Eq, Serialize, Deserialize)]
pub enum AxisBinding {
    Zoom
}

impl fmt::Display for ActionBinding {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}

impl fmt::Display for AxisBinding {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}


#[derive(Debug)]
pub struct MovementBindingTypes;

impl BindingTypes for MovementBindingTypes {
    type Axis = AxisBinding;
    type Action = ActionBinding;
}

const MOVE_ACTIONS: &'static [&'static ActionBinding] = &[
    &ActionBinding::Forward,
    &ActionBinding::Backward,
    &ActionBinding::StrafeLeft,
    &ActionBinding::StrafeRight,
];

pub fn move_direction_from_binding(binding: &ActionBinding) -> Option<MoveDirection> {
    match binding {
        ActionBinding::Forward => Some(MoveDirection::Forward),
        ActionBinding::Backward => Some(MoveDirection::Backward),
        ActionBinding::StrafeLeft => Some(MoveDirection::StrafeLeft),
        ActionBinding::StrafeRight => Some(MoveDirection::StrafeRight),
        _ => None,
    }
}

