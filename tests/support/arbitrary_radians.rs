extern crate quickcheck;

use {PI, Scalar};

#[derive(Copy, Clone, Debug)]
pub struct Radians(Scalar);

impl From<Radians> for Scalar {
    fn from(radians: Radians) -> Self {
        radians.0
    }
}

impl quickcheck::Arbitrary for Radians {
    fn arbitrary<G: quickcheck::Gen>(random: &mut G) -> Self {
        Radians(random.gen_range(-PI, PI))
    }
}
