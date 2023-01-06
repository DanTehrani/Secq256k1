pub mod affine;
pub mod field;
pub mod scalar;

pub use affine::AffinePoint;
use affine::AffinePointCore;
pub use primeorder::elliptic_curve;
pub use primeorder::elliptic_curve::bigint::U256;

use field::FieldElement;
use primeorder::elliptic_curve::{AffineArithmetic, Curve, ProjectiveArithmetic, ScalarArithmetic};
use primeorder::{PrimeCurve, PrimeCurveParams};
pub use scalar::Scalar;

pub type EncodedPoint = primeorder::elliptic_curve::sec1::EncodedPoint<Secq256K1>;
pub type FieldBytes = primeorder::elliptic_curve::FieldBytes<Secq256K1>;
pub type ProjectivePoint = primeorder::ProjectivePoint<Secq256K1>;

#[derive(Copy, Clone, Debug, Default, Eq, PartialEq, PartialOrd, Ord)]
pub struct Secq256K1;

impl Curve for Secq256K1 {
    type UInt = U256;

    const ORDER: U256 =
        U256::from_be_hex("fffffffffffffffffffffffffffffffffffffffffffffffffffffffefffffc2f");
}

impl PrimeCurveParams for Secq256K1 {
    type FieldElement = FieldElement;

    const ZERO: FieldElement = FieldElement::ZERO;
    const ONE: FieldElement = FieldElement::ONE;

    const EQUATION_A: FieldElement = FieldElement::ZERO;

    const EQUATION_B: FieldElement = FieldElement::from_be_hex(
        "0000000000000000000000000000000000000000000000000000000000000007",
    );

    const GENERATOR: (FieldElement, FieldElement) = (
        FieldElement::from_be_hex(
            "76c39f5585cb160eb6b06c87a2ce32e23134e45a097781a6a24288e37702eda6",
        ),
        FieldElement::from_be_hex(
            "3ffc646c7b2918b5dc2d265a8e82a7f7d18983d26e8dc055a4120ddad952677f",
        ),
    );
}

impl PrimeCurve for Secq256K1 {}

impl AffineArithmetic for Secq256K1 {
    type AffinePoint = AffinePointCore;
}

impl ProjectiveArithmetic for Secq256K1 {
    type ProjectivePoint = ProjectivePoint;
}

impl ScalarArithmetic for Secq256K1 {
    type Scalar = Scalar;
}
