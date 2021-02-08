use amethyst::core::math::Point2;

#[derive(Copy, Clone)]
pub struct CursorPosition {
    pub pos: Point2<f32>
}

impl Default for CursorPosition {
    fn default() -> Self {
        CursorPosition {pos: Point2::new(0.0, 0.0)}
    }
}