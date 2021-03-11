
use amethyst::ecs::prelude::{Component, DenseVecStorage};
use std::time::Duration;

#[derive(Debug)]
pub struct TimeLimit
{
    pub time_to_live: Duration,
    pub timing_start: Duration,
}

impl Component for TimeLimit {
    type Storage = DenseVecStorage<Self>;
}

impl TimeLimit
{
    pub fn new(secs_to_live: f32, timing_start: Duration) -> Self {
        TimeLimit {
            time_to_live: Duration::from_secs_f32(secs_to_live),
            timing_start
        }
    }

    pub fn timing_end(&self) -> Duration {
        self.timing_start + self.time_to_live
    }
}
