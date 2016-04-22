extern crate quickcheck;

use maths::{UnitVec3D, Vec3D};

impl quickcheck::Arbitrary for UnitVec3D {
    fn arbitrary<G: quickcheck::Gen>(random: &mut G) -> Self {
        Vec3D::arbitrary(random).normalize()
    }
}
