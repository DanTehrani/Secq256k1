use std::ops::{Add, Mul, Sub};

use crate::{EncodedPoint, Scalar};
use primeorder::elliptic_curve::sec1::FromEncodedPoint;
use primeorder::elliptic_curve::sec1::ToCompactEncodedPoint;
use primeorder::elliptic_curve::subtle::CtOption;

use super::{ProjectivePoint, Secq256K1};

pub type AffinePointCore = primeorder::AffinePoint<Secq256K1>;

#[derive(Copy, Clone, Debug, Default, Eq, PartialEq)]
pub struct AffinePoint(pub AffinePointCore);

impl Mul<Scalar> for AffinePoint {
    type Output = AffinePoint;

    fn mul(self, rhs: Scalar) -> Self::Output {
        AffinePoint((self.0 * rhs).into())
    }
}

impl Mul<Scalar> for &AffinePoint {
    type Output = AffinePoint;

    fn mul(self, rhs: Scalar) -> Self::Output {
        AffinePoint((self.0 * rhs).into())
    }
}

impl Add<AffinePoint> for AffinePoint {
    type Output = AffinePoint;

    fn add(self, rhs: AffinePoint) -> Self::Output {
        AffinePoint((ProjectivePoint::from(self.0) + rhs.0).into())
    }
}

impl Sub<AffinePoint> for AffinePoint {
    type Output = AffinePoint;

    fn sub(self, rhs: AffinePoint) -> Self::Output {
        AffinePoint((ProjectivePoint::from(self.0) - rhs.0).into())
    }
}

impl AffinePoint {
    pub fn identity() -> Self {
        AffinePoint(AffinePointCore::IDENTITY)
    }

    pub fn generator() -> Self {
        AffinePoint(AffinePointCore::GENERATOR)
    }

    pub fn compress(&self) -> EncodedPoint {
        self.0.to_compact_encoded_point().unwrap()
    }

    pub fn decompress(bytes: EncodedPoint) -> CtOption<Self> {
        AffinePointCore::from_encoded_point(&bytes).map(AffinePoint)
    }
}

impl From<ProjectivePoint> for AffinePoint {
    fn from(p: ProjectivePoint) -> Self {
        AffinePoint(p.into())
    }
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn add() {
        let a = AffinePoint::generator();
        println!("{:?}", a + a);
    }
}
