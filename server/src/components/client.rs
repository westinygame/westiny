use crate::resources::ClientID;
use derive_new::new;
use amethyst::core::ecs::{Component, VecStorage};

#[derive(new, Copy, Clone, Debug)]
pub struct Client {
    id: ClientID
}

impl Component for Client {
    type Storage = VecStorage<Self>;
}

impl Client {
    pub fn id(&self) -> &ClientID {
        &self.id
    }
}