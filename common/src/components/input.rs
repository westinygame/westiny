use crate::metric_dimension::length::{Meter, MeterVec2};
use bevy::ecs::component::Component;
use bitflags;
use serde::{Deserialize, Serialize};

const SELECTIONS: [InputFlags; 5] = [
    InputFlags::SELECT1,
    InputFlags::SELECT2,
    InputFlags::SELECT3,
    InputFlags::SELECT4,
    InputFlags::SELECT5,
];

bitflags::bitflags! {
    #[derive(Serialize, Deserialize)]
    pub struct InputFlags: u16 {
        const NOP =      0b0000_0000_0000_0000;
        const FORWARD =  0b0000_0000_0000_0001;
        const BACKWARD = 0b0000_0000_0000_0010;
        const LEFT =     0b0000_0000_0000_0100;
        const RIGHT =    0b0000_0000_0000_1000;
        const SHOOT =    0b0000_0000_0001_0000;
        const USE =      0b0000_0000_0010_0000;
        const RUN =      0b0000_0000_0100_0000;
        const RELOAD =   0b0000_0000_1000_0000;
        const SELECT1 =  0b0000_0001_0000_0000;
        const SELECT2 =  0b0000_0010_0000_0000;
        const SELECT3 =  0b0000_0100_0000_0000;
        const SELECT4 =  0b0000_1000_0000_0000;
        const SELECT5 =  0b0001_0000_0000_0000;
    }

}

#[derive(Copy, Clone, Debug, Serialize, Deserialize, PartialEq, Component)]
pub struct Input {
    pub flags: InputFlags,
    pub cursor: MeterVec2,
}

impl Input {
    /// returns the first SELECT value if any of them is active. Otherwise returns None
    pub fn get_selection(&self) -> Option<&InputFlags> {
        SELECTIONS
            .iter()
            .find(|&select| self.flags.intersects(*select))
    }
}

impl Default for Input {
    fn default() -> Self {
        Input {
            flags: InputFlags::NOP,
            cursor: MeterVec2 {
                x: Meter(0.0),
                y: Meter(0.0),
            },
        }
    }
}
