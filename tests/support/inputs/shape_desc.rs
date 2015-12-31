extern crate quickcheck;

use {entities, Scalar};

#[derive(Clone, Debug)]
pub enum ShapeDesc {
    Cuboid(Scalar, Scalar, Scalar),
}

impl ShapeDesc {
    pub fn to_value(self) -> entities::ShapeDesc {
        match self {
            ShapeDesc::Cuboid(x, y, z) => {
                entities::ShapeDesc::Cuboid(x, y, z)
            },
        }
    }
}

impl quickcheck::Arbitrary for ShapeDesc {
    fn arbitrary<G: quickcheck::Gen>(generator: &mut G) -> Self {
        match generator.next_u32() % 1 {
            _ => {
                ShapeDesc::Cuboid(
                    generator.gen_range(1.1, 10.0),
                    generator.gen_range(1.1, 10.0),
                    generator.gen_range(1.1, 10.0),
                )
            },
        }
    }
}
