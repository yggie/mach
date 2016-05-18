extern crate quickcheck;

use Scalar;

use tests::support::TEST_SCALAR_BOUNDS;

#[derive(Copy, Clone, Debug)]
pub struct PositiveScalar(Scalar);

impl From<PositiveScalar> for Scalar {
    fn from(positive_scalar: PositiveScalar) -> Self {
        positive_scalar.0
    }
}

impl quickcheck::Arbitrary for PositiveScalar {
    fn arbitrary<G: quickcheck::Gen>(random: &mut G) -> Self {
        PositiveScalar(random.gen_range(0.0, TEST_SCALAR_BOUNDS).abs())
    }
}
