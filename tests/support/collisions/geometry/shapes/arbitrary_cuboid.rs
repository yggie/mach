extern crate quickcheck;

use TOLERANCE;
use collisions::geometry::shapes::Cuboid;

use tests::support::TEST_SCALAR_BOUNDS;

impl quickcheck::Arbitrary for Cuboid {
    fn arbitrary<G: quickcheck::Gen>(random: &mut G) -> Self {
        Cuboid::new(
            random.gen_range(TOLERANCE, TEST_SCALAR_BOUNDS),
            random.gen_range(TOLERANCE, TEST_SCALAR_BOUNDS),
            random.gen_range(TOLERANCE, TEST_SCALAR_BOUNDS),
        )
    }
}
