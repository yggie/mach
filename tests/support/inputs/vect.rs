extern crate quickcheck;

use std::fmt;

use {maths, Scalar};

#[derive(Clone)]
pub struct Vect(pub Scalar, pub Scalar, pub Scalar);

impl Into<maths::Vect> for Vect {
    fn into(self) -> maths::Vect {
        maths::Vect::new(self.0, self.1, self.2)
    }
}

impl quickcheck::Arbitrary for Vect {
    fn arbitrary<G: quickcheck::Gen>(random: &mut G) -> Self {
        Vect(
            random.gen_range(-100.0, 100.0),
            random.gen_range(-100.0, 100.0),
            random.gen_range(-100.0, 100.0),
        )
    }
}

impl fmt::Debug for Vect {
    fn fmt(&self, formatter: &mut fmt::Formatter) -> Result<(), fmt::Error> {
        write!(formatter, "Vect({}, {}, {})", self.0, self.1, self.2)
    }
}
