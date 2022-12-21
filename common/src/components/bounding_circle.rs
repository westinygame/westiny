use crate::metric_dimension::length::Meter;
use bevy::prelude::*;

#[derive(Default, Copy, Clone, Debug, Component, Reflect)]
#[reflect(Component)]
pub struct BoundingCircle {
    pub radius: Meter,
}
