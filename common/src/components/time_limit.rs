use std::time::Duration;
use crate::metric_dimension::Second;
use bevy::ecs::component::Component;

#[derive(Debug, Component)]
pub struct Lifespan
{
    pub living_until: Duration,
}

impl Lifespan
{
    pub fn new(secs_to_live: Second, timing_start: Duration) -> Self {
        Lifespan {
            living_until: timing_start + secs_to_live.into_duration(),
        }
    }
}
