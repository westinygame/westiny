use crate::metric_dimension::length::Meter;
use bevy::ecs::component::Component;

#[derive(Copy, Clone, Debug, Component)]
pub struct BoundingCircle {
    pub radius: Meter,
}
