extern crate quickcheck;

use maths::_2d::Vec2D;

#[test]
fn it_always_has_a_length_of_one() {
    fn property(random_vec: Vec2D) {
        let unit_vec = random_vec.normalize();

        assert_approx_eq!(Vec2D::from(unit_vec).squared_length(), 1.0);
    }

    quickcheck::quickcheck(property as fn(Vec2D));
}
