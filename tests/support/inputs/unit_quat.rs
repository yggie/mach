extern crate quickcheck;

use std::fmt;

use mach::maths::Quat;

use support::inputs;

#[derive(Clone)]
pub struct UnitQuat {
    pub axis: inputs::UnitVect,
    pub angle: inputs::Radians,
}

impl UnitQuat {
    pub fn identity() -> UnitQuat {
        UnitQuat {
            axis: inputs::UnitVect { values: (1.0, 0.0, 0.0) },
            angle: inputs::Radians(0.0),
        }
    }

    pub fn to_value(self) -> Quat {
        Quat::new_from_axis_angle(self.axis.to_value(), self.angle.to_value())
    }
}

impl quickcheck::Arbitrary for UnitQuat {
    fn arbitrary<G: quickcheck::Gen>(random: &mut G) -> Self {
        UnitQuat {
            axis: quickcheck::Arbitrary::arbitrary(random),
            angle: quickcheck::Arbitrary::arbitrary(random),
        }
    }
}

impl fmt::Debug for UnitQuat {
    fn fmt(&self, formatter: &mut fmt::Formatter) -> Result<(), fmt::Error> {
        write!(formatter, "UnitQuat({:?}, {:?})", self.axis, self.angle)
    }
}
