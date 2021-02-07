use std::ops::Deref;

#[derive(Copy, Clone, Eq, PartialEq, Debug)]
pub struct NetworkId(u64);

impl NetworkId {
    pub fn new(id: u64) -> Self {
        NetworkId(id)
    }

    pub fn get(&self) -> &u64 {
        &self.0
    }
}

impl Deref for NetworkId {
    type Target = u64;

    fn deref(&self) -> &Self::Target {
        self.get()
    }
}