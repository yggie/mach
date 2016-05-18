extern crate quickcheck;

use maths::_2d::Vec2D;

use tests::support::TEST_SCALAR_BOUNDS;

impl quickcheck::Arbitrary for Vec2D {
    fn arbitrary<G: quickcheck::Gen>(random: &mut G) -> Self {
        Vec2D::new(
            random.gen_range(-TEST_SCALAR_BOUNDS, TEST_SCALAR_BOUNDS),
            random.gen_range(-TEST_SCALAR_BOUNDS, TEST_SCALAR_BOUNDS),
        )
    }
}
