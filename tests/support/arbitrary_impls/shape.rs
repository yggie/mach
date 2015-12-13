extern crate quickcheck;

use Scalar;
use shapes::{Cuboid, Shape};

impl quickcheck::Arbitrary for Box<Shape> {
    fn arbitrary<G: quickcheck::Gen>(generator: &mut G) -> Self {
        match generator.next_u32() % 1 {
            0 => {
                Box::new(Cuboid::new(
                    generator.next_f32() as Scalar,
                    generator.next_f32() as Scalar,
                    generator.next_f32() as Scalar,
                ))
            },
        }
    }
}
