extern crate quickcheck;

use maths::_2d::{UnitVec2D, Vec2D};

#[test]
fn it_always_has_a_length_of_one() {
    fn property(vec: Vec2D) {
        let unit_vec = UnitVec2D::from_vec(&vec);

        assert_approx_eq!(unit_vec.vec().squared_length(), 1.0);
    }

    quickcheck::quickcheck(property as fn(Vec2D));
}
