extern crate quickcheck;

use maths::Vec3D;

impl quickcheck::Arbitrary for Vec3D {
    fn arbitrary<G: quickcheck::Gen>(random: &mut G) -> Self {
        Vec3D::new(
            random.gen_range(-100.0, 100.0),
            random.gen_range(-100.0, 100.0),
            random.gen_range(-100.0, 100.0),
        )
    }
}
