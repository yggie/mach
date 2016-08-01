extern crate quickcheck;

use TOLERANCE;
use collisions::geometry::shapes::Sphere;

use tests::support::TEST_SCALAR_BOUNDS;

impl quickcheck::Arbitrary for Sphere {
    fn arbitrary<G: quickcheck::Gen>(random: &mut G) -> Self {
        Sphere::new(random.gen_range(TOLERANCE, TEST_SCALAR_BOUNDS))
    }
}
