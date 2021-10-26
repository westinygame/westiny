use crate::metric_dimension::length::MeterVec2;

#[derive(Copy, Clone)]
pub struct CursorPosition {
    pub pos: MeterVec2
}

impl Default for CursorPosition {
    fn default() -> Self {
        CursorPosition { pos: MeterVec2::from_raw(0.0, 0.0) }
    }
}