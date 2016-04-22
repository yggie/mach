extern crate quickcheck;

use std::fmt;

use {Scalar, Vec3D};

#[derive(Clone)]
pub struct UnitVect {
    pub values: (Scalar, Scalar, Scalar),
}

impl UnitVect {
    pub fn as_vect(self) -> Vec3D {
        Into::<Vec3D>::into(self)
    }
}

impl Into<Vec3D> for UnitVect {
    fn into(self) -> Vec3D {
        Vec3D::new(self.values.0, self.values.1, self.values.2).normalize()
    }
}

impl quickcheck::Arbitrary for UnitVect {
    fn arbitrary<G: quickcheck::Gen>(random: &mut G) -> Self {
        let value = Vec3D::new(
            random.gen_range(-1.0, 1.0),
            random.gen_range(-1.0, 1.0),
            random.gen_range(-1.0, 1.0),
        ).normalize();

        UnitVect {
            values: (value.x, value.y, value.z),
        }
    }
}

impl fmt::Debug for UnitVect {
    fn fmt(&self, formatter: &mut fmt::Formatter) -> Result<(), fmt::Error> {
        write!(formatter, "UnitVect({}, {}, {})", self.values.0, self.values.1, self.values.2)
    }
}
