extern crate quickcheck;

use geometry::_2d::Ray2D;

impl quickcheck::Arbitrary for Ray2D {
    fn arbitrary<G: quickcheck::Gen>(random: &mut G) -> Self {
        Ray2D::new(
            quickcheck::Arbitrary::arbitrary(random),
            quickcheck::Arbitrary::arbitrary(random),
        )
    }
}
