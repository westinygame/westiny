use crate::resources::ClientID;
use derive_new::new;

#[derive(Copy, Clone, new)]
pub struct Client {
    pub id: ClientID
}

impl Client {
    pub fn id(&self) -> &ClientID {
        &self.id
    }
}

