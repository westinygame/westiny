use std::ops::{Mul, Div, AddAssign, Neg};
use std::time::Duration;
use crate::metric_dimension::length::Meter;
use serde::{Serialize, Deserialize};
use amethyst::core::math::{Vector2, Rotation2};
use std::fmt::{Debug, Display, Formatter};
use num_derive::{Float, Num, NumCast, NumOps, ToPrimitive, One, Zero};

macro_rules! impl_trait {
    (impl $trait:ident :: $method:ident ::< $other:ty > for $base:ident -> $output:ident) => {
        impl $trait<$other> for $base {
            type Output = $output;

            fn $method(self, rhs: $other) -> Self::Output {
                $output(self.0.$method(rhs.0))
            }
        }

        impl $trait<Vector2<$other>> for $base {
            type Output = Vector2<$output>;

            fn $method(self, rhs: Vector2<$other>) -> Self::Output {
                Vector2::new(self.$method(rhs.x), self.$method(rhs.y))
            }
        }
    }
}

impl_trait!{ impl Div::div::<Second> for Meter -> MeterPerSec }
impl_trait!{ impl Mul::mul::<MeterPerSec> for Second -> Meter }
impl_trait!{ impl Div::div::<MeterPerSec> for Meter -> Second }

const PIXEL_PER_METER: u16 = 32;

pub mod length {
    use std::ops::{Mul, Neg};
    use amethyst::core::math::{Vector3, Vector2};
    use super::*;
    use serde::Deserialize;
    use num_derive::{Float, Num, NumCast, NumOps, ToPrimitive, One, Zero};

    #[derive(Debug, Copy, Clone, Serialize, Deserialize, PartialEq, PartialOrd,
    Float, Num, NumCast, NumOps, ToPrimitive, One, Zero)]
    pub struct Meter(pub f32);

    impl Neg for Meter {
        type Output = Self;

        fn neg(self) -> Self::Output {
            Meter(self.0.neg())
        }
    }

    impl Meter {
        pub fn into_pixel(self) -> f32 {
            self.0 * (PIXEL_PER_METER as f32)
        }

        pub fn from_pixel(pixel: f32) -> Self {
            Meter(pixel / (PIXEL_PER_METER as f32))
        }
    }

    impl Mul<Vector3<f32>> for Meter {
        type Output = Vector3<Meter>;

        /// The z coordinate will not be multiplied
        fn mul(self, rhs: Vector3<f32>) -> Self::Output {
            Vector3::new(Meter(self.0 * rhs.x), Meter(self.0 * rhs.y), Meter::from_pixel(rhs.z))
        }
    }

    impl Mul<f32> for Meter {
        type Output = Meter;

        fn mul(self, rhs: f32) -> Self::Output {
            Meter(self.0 * rhs)
        }
    }

    impl Mul<Vector2<f32>> for Meter {
        type Output = Vector2<Meter>;

        fn mul(self, rhs: Vector2<f32>) -> Self::Output {
            Vector2::new(self * rhs.x, self * rhs.y)
        }
    }

    pub fn magnitude(vector: Vector2<Meter>) -> Meter {
        let pixel_vec = Vector2::new(vector.x.into_pixel(), vector.y.into_pixel());
        Meter::from_pixel(pixel_vec.magnitude())
    }

    pub fn normalize(vector: Vector2<Meter>) -> Vector2<f32> {
        let pixel_vec = Vector2::new(vector.x.into_pixel(), vector.y.into_pixel());
        let magnitude = pixel_vec.magnitude();
        pixel_vec / magnitude
    }
}

#[derive(Debug, Copy, Clone, Serialize, Deserialize, PartialEq)]
pub struct Second(pub f32);

impl Second {
    pub fn into_duration(self) -> Duration {
        Duration::from_secs_f32(self.0)
    }
}

impl From<Duration> for Second {
    fn from(duration: Duration) -> Self {
        Second(duration.as_secs_f32())
    }
}

#[derive(Debug, Copy, Clone, Serialize, Deserialize, PartialEq, PartialOrd,
Float, Num, NumCast, NumOps, ToPrimitive, One, Zero)]
pub struct MeterPerSec(pub f32);

impl MeterPerSec {
    pub fn from_pixel_per_sec(pixel_per_sec: f32) -> Self {
        MeterPerSec(pixel_per_sec / (PIXEL_PER_METER as f32))
    }
}

impl Neg for MeterPerSec {
    type Output = Self;
    fn neg(self) -> Self::Output {
        MeterPerSec(self.0.neg())
    }
}

impl Mul<Vector2<f32>> for MeterPerSec {
    type Output = Vector2<MeterPerSec>;

    fn mul(self, rhs: Vector2<f32>) -> Self::Output {
        Vector2::new(MeterPerSec(self.0 * rhs.x), MeterPerSec(self.0 * rhs.y))
    }
}

impl Div<f32> for MeterPerSec {
    type Output = MeterPerSec;

    fn div(self, rhs: f32) -> Self::Output {
        MeterPerSec(self.0 / (rhs as f32))
    }
}

impl AddAssign<MeterPerSec> for MeterPerSec {
    fn add_assign(&mut self, rhs: MeterPerSec) {
        self.0.add_assign(rhs.0)
    }
}

impl Display for MeterPerSec {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} m/s", self.0)
    }
}

pub fn rotate(velocity: Vector2<MeterPerSec>, rotation: Rotation2<f32>) -> Vector2<MeterPerSec> {
    let rotated = rotation * Vector2::new(velocity.x.0, velocity.y.0);
    Vector2::new(MeterPerSec(rotated.x), MeterPerSec(rotated.y))
}

pub fn to_meter_vec(pixel_vec: Vector2<f32>) -> Vector2<Meter> {
    Vector2::new(Meter::from_pixel(pixel_vec.x), Meter::from_pixel(pixel_vec.y))
}