use bevy::ecs::component::Component;

#[derive(Copy, Clone, Debug, Component)]
pub struct Eliminated {
    pub elimination_time_sec: f64,
}
