extern crate quickcheck;

use mach::Scalar;
use mach::maths;
use mach::maths::{Quat, Vector};

use support::inputs;

#[derive(Clone, Debug)]
pub struct Transform {
    axis: (Scalar, Scalar, Scalar),
    angle: inputs::Radians,
    translation: (Scalar, Scalar, Scalar),
}

impl Transform {
    pub fn to_value(self) -> maths::Transform {
        let axis = Vector::new(self.axis.0, self.axis.1, self.axis.2);

        maths::Transform::new(
            Vector::new(self.translation.0, self.translation.1, self.translation.2),
            Quat::new_from_axis_angle(axis, self.angle.to_value()),
        )
    }
}

impl quickcheck::Arbitrary for Transform {
    fn arbitrary<G: quickcheck::Gen>(generator: &mut G) -> Self {
        let vector: (Scalar, Scalar, Scalar) = quickcheck::Arbitrary::arbitrary(generator);
        let length = (vector.0*vector.0 + vector.1*vector.1 + vector.2*vector.2).sqrt();

        Transform {
            axis: (vector.0 / length, vector.1 / length, vector.2 / length),
            angle: quickcheck::Arbitrary::arbitrary(generator),
            translation: quickcheck::Arbitrary::arbitrary(generator),
        }
    }
}
