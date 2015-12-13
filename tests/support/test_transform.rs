extern crate quickcheck;

use mach::Scalar;
use mach::maths::{Transform, Quat, Vector};

#[derive(Clone, Debug)]
pub struct TestTransform {
    axis: (Scalar, Scalar, Scalar),
    angle: Scalar,
    vector: (Scalar, Scalar, Scalar),
}

impl TestTransform {
    pub fn as_transform(self) -> Transform {
        let axis = Vector::new(self.axis.0, self.axis.1, self.axis.2);

        Transform::new(
            Vector::new(self.vector.0, self.vector.1, self.vector.2),
            Quat::new_from_axis_angle(axis, self.angle),
        )
    }
}

impl quickcheck::Arbitrary for TestTransform {
    fn arbitrary<G: quickcheck::Gen>(generator: &mut G) -> Self {
        TestTransform {
            axis: quickcheck::Arbitrary::arbitrary(generator),
            angle: quickcheck::Arbitrary::arbitrary(generator),
            vector: quickcheck::Arbitrary::arbitrary(generator),
        }
    }
}
