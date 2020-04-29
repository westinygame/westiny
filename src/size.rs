pub type SizeUnit = f32;

pub struct UnitToPixelCalculator {
    multiplier: u32,
}

impl UnitToPixelCalculator {
    pub fn new(multiplier: u32) -> UnitToPixelCalculator {
        UnitToPixelCalculator {
            multiplier
        }
    }

    pub fn to_pixels(&self, size_unit: SizeUnit) -> u32 {
        (self.multiplier as f32 * size_unit) as u32
    }

    pub fn to_units(&self, size_pixel: u32) -> SizeUnit {
        (size_pixel / self.multiplier) as SizeUnit
    }
}
