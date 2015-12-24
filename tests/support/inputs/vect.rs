extern crate quickcheck;

use std::fmt;

use {Scalar, Vector};

#[derive(Clone)]
pub struct Vect {
    pub values: (Scalar, Scalar, Scalar),
}

impl Vect {
    pub fn to_value(self) -> Vector {
        Vector::new(self.values.0, self.values.1, self.values.2)
    }
}

impl quickcheck::Arbitrary for Vect {
    fn arbitrary<G: quickcheck::Gen>(random: &mut G) -> Self {
        Vect {
            values: (
                random.gen_range(-100.0, 100.0),
                random.gen_range(-100.0, 100.0),
                random.gen_range(-100.0, 100.0),
            ),
        }
    }
}

impl fmt::Debug for Vect {
    fn fmt(&self, formatter: &mut fmt::Formatter) -> Result<(), fmt::Error> {
        write!(formatter, "Vect({}, {}, {})", self.values.0, self.values.1, self.values.2)
    }
}
