extern crate quickcheck;

use mach::Scalar;
use mach::maths;
use mach::maths::{Quat, Vector};

#[derive(Clone, Debug)]
pub struct Transform {
    axis: (Scalar, Scalar, Scalar),
    angle: Scalar,
    vector: (Scalar, Scalar, Scalar),
}

impl Transform {
    pub fn to_object(self) -> maths::Transform {
        let axis = Vector::new(self.axis.0, self.axis.1, self.axis.2);

        maths::Transform::new(
            Vector::new(self.vector.0, self.vector.1, self.vector.2),
            Quat::new_from_axis_angle(axis, self.angle),
        )
    }
}

impl quickcheck::Arbitrary for Transform {
    fn arbitrary<G: quickcheck::Gen>(generator: &mut G) -> Self {
        Transform {
            axis: quickcheck::Arbitrary::arbitrary(generator),
            angle: quickcheck::Arbitrary::arbitrary(generator),
            vector: quickcheck::Arbitrary::arbitrary(generator),
        }
    }
}
