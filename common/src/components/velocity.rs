use crate::metric_dimension::MeterPerSecVec2;
use bevy::ecs::component::Component;


#[derive(Debug, Component)]
pub struct Velocity(pub MeterPerSecVec2);

impl Default for Velocity {
    fn default() -> Self {
        Velocity(MeterPerSecVec2::from_raw(0.0, 0.0))
    }
}
