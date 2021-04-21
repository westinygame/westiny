use amethyst::core::math::Point2;
use crate::metric_dimension::length::Meter;

#[derive(Copy, Clone)]
pub struct CursorPosition {
    pub pos: Point2<Meter>
}

impl Default for CursorPosition {
    fn default() -> Self {
        CursorPosition {pos: Point2::new(Meter(0.0), Meter(0.0))}
    }
}