use amethyst::core::math::Point2;
use amethyst::ecs::prelude::{Component, DenseVecStorage};
use serde::{Serialize, Deserialize};

use bitflags;

bitflags::bitflags! {
    #[derive(Serialize, Deserialize)]
    pub struct InputFlags: u16 {
        const NOP =      0b0000_0000_0000_0000;
        const FORWARD =  0b0000_0000_0000_0001;
        const BACKWARD = 0b0000_0000_0000_0010;
        const LEFT =     0b0000_0000_0000_0100;
        const RIGHT =    0b0000_0000_0000_1000;
        const SHOOT =    0b0000_0000_0001_0000;
        //const USE =    0b0000_0000_0010_0000;
        const RUN =      0b0000_0000_0100_0000;
        const RELOAD =   0b0000_0000_1000_0000;
        const SELECT1 =  0b0000_0001_0000_0000;
        const SELECT2 =  0b0000_0010_0000_0000;
        const SELECT3 =  0b0000_0100_0000_0000;
        const SELECT4 =  0b0000_1000_0000_0000;
        const SELECT5 =  0b0001_0000_0000_0000;
    }

}

#[derive(Copy, Clone, Debug, Serialize, Deserialize, PartialEq)]
pub struct Input
{
    pub flags : InputFlags,
    pub cursor : Point2<f32>
}

impl Default for Input
{
    fn default() -> Self {
        Input{
            flags: InputFlags::NOP,
            cursor: Point2::new(0.0, 0.0),
        }
    }
}

impl Component for Input {
    type Storage = DenseVecStorage<Self>;
}
