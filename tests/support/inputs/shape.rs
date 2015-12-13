extern crate quickcheck;

use mach::Scalar;
use mach::shapes;

#[derive(Clone, Debug)]
pub enum Shape {
    Cuboid(Scalar, Scalar, Scalar),
}

impl Shape {
    pub fn to_object(self) -> Box<shapes::Shape> {
        match self {
            Shape::Cuboid(x, y, z) => {
                Box::new(shapes::Cuboid::new(x, y, z))
            },
        }
    }
}

impl quickcheck::Arbitrary for Shape {
    fn arbitrary<G: quickcheck::Gen>(generator: &mut G) -> Self {
        match generator.next_u32() % 1 {
            _ => {
                Shape::Cuboid(
                    100.0 * generator.next_f32() as Scalar,
                    100.0 * generator.next_f32() as Scalar,
                    100.0 * generator.next_f32() as Scalar,
                )
            },
        }
    }
}
