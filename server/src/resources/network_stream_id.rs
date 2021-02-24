#[derive(Copy, Clone, Debug)]
#[repr(u8)]
pub enum StreamId {
    EntityStateUpdate,
}

impl Into<Option<u8>> for StreamId {
    fn into(self) -> Option<u8> {
        Some(self as u8)
    }
}
