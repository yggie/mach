extern crate quickcheck;

use maths::UnitQuat;

impl quickcheck::Arbitrary for UnitQuat {
    fn arbitrary<G: quickcheck::Gen>(random: &mut G) -> Self {
        UnitQuat::from_axis_angle(
            quickcheck::Arbitrary::arbitrary(random),
            quickcheck::Arbitrary::arbitrary(random),
        )
    }
}
