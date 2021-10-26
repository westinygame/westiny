use crate::metric_dimension::MeterPerSecVec2;

#[derive(Debug)]
pub struct Velocity(pub MeterPerSecVec2);

impl Default for Velocity {
    fn default() -> Self {
        Velocity(MeterPerSecVec2::from_raw(0.0, 0.0))
    }
}
