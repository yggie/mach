extern crate quickcheck;

use mach::Scalar;
use mach::shapes::{Cuboid, Shape};

#[derive(Clone, Debug)]
pub enum TestShape {
    Cuboid(Scalar, Scalar, Scalar),
}

impl TestShape {
    pub fn as_shape(self) -> Box<Shape> {
        match self {
            TestShape::Cuboid(x, y, z) => {
                Box::new(Cuboid::new(x, y, z))
            },
        }
    }
}

impl quickcheck::Arbitrary for TestShape {
    fn arbitrary<G: quickcheck::Gen>(generator: &mut G) -> Self {
        match generator.next_u32() % 1 {
            _ => {
                TestShape::Cuboid(
                    generator.next_f32() as Scalar,
                    generator.next_f32() as Scalar,
                    generator.next_f32() as Scalar,
                )
            },
        }
    }
}
