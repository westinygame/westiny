use amethyst::core::math::Point2;
use amethyst::ecs::prelude::{Component, DenseVecStorage};
use serde::{Serialize, Deserialize};

#[derive(Copy, Clone, Debug, Serialize, Deserialize)]
pub struct Input
{
    // bool means is_down? should we use enum bits?
    pub forward : bool,
    pub backward : bool,
    pub left : bool,
    pub right : bool,
    pub shoot : bool,
    pub cursor : Point2<f32>
}

impl Default for Input
{
    fn default() -> Self {
        Input{
            forward: false,
            backward: false,
            left: false,
            right: false,
            shoot: false,
            cursor: Point2::new(0.0, 0.0),
        }
    }
}

impl Component for Input {
    type Storage = DenseVecStorage<Self>;
}
