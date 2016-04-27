extern crate quickcheck;

use geometry::_2d::Point2D;

impl quickcheck::Arbitrary for Point2D {
    fn arbitrary<G: quickcheck::Gen>(random: &mut G) -> Self {
        Point2D(quickcheck::Arbitrary::arbitrary(random))
    }
}
