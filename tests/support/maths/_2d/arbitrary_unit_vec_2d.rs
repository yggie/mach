extern crate quickcheck;

use maths::_2d::{UnitVec2D, Vec2D};

impl quickcheck::Arbitrary for UnitVec2D {
    fn arbitrary<G: quickcheck::Gen>(random: &mut G) -> Self {
        Vec2D::arbitrary(random).normalize()
    }
}
