extern crate quickcheck;

use maths::_2d::Vec2D;

impl quickcheck::Arbitrary for Vec2D {
    fn arbitrary<G: quickcheck::Gen>(random: &mut G) -> Self {
        Vec2D::new(
            quickcheck::Arbitrary::arbitrary(random),
            quickcheck::Arbitrary::arbitrary(random),
        )
    }
}
