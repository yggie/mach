extern crate quickcheck;

use std::fmt;

use mach::{PI, Scalar};

#[derive(Clone, Copy)]
pub struct Radians(Scalar);

impl Radians {
    pub fn to_value(self) -> Scalar {
        self.0 * PI
    }
}

impl quickcheck::Arbitrary for Radians {
    fn arbitrary<G: quickcheck::Gen>(generator: &mut G) -> Self {
        Radians(2.0 * (generator.next_f32() - 0.5))
    }
}

impl fmt::Debug for Radians {
    fn fmt(&self, formatter: &mut fmt::Formatter) -> Result<(), fmt::Error> {
        write!(formatter, "Radians({}π)", self.0)
    }
}
