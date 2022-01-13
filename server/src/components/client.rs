use crate::resources::ClientID;
use derive_new::new;
use bevy::ecs::component::Component;

#[derive(Copy, Clone, new, Component)]
pub struct Client {
    pub id: ClientID
}

