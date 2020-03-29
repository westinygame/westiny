pub type SizeUnit = u32;

pub struct UnitToPixelCalculator {
    multiplier: u32,
}

impl UnitToPixelCalculator {
    pub fn new(multiplier: u32) -> UnitToPixelCalculator {
        UnitToPixelCalculator {
            multiplier
        }
    }

    pub fn to_pixels(&self, size_unit: &SizeUnit) -> u32 {
        self.multiplier * size_unit
    }
}
