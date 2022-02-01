use crate::resources::ClientID;
use bevy::ecs::component::Component;
use derive_new::new;

#[derive(Copy, Clone, new, Component)]
pub struct Client {
    pub id: ClientID,
}
