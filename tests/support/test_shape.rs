extern crate quickcheck;

use mach::Scalar;
use mach::shapes::{Cuboid, Shape};

#[derive(Clone, Debug)]
pub enum TestShape {
    Cuboid {
        depth: Scalar,
        width: Scalar,
        height: Scalar,
    },
}

impl TestShape {
    pub fn as_shape(self) -> Box<Shape> {
        match self {
            TestShape::Cuboid { width, depth, height } => {
                Box::new(Cuboid::new(width, height, depth))
            },
        }
    }
}

impl quickcheck::Arbitrary for TestShape {
    fn arbitrary<G: quickcheck::Gen>(generator: &mut G) -> Self {
        match generator.next_u32() % 1 {
            _ => {
                TestShape::Cuboid {
                    width: generator.next_f32() as Scalar,
                    depth: generator.next_f32() as Scalar,
                    height: generator.next_f32() as Scalar,
                }
            },
        }
    }
}
