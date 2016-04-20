extern crate quickcheck;

use maths::_2d::Vec2D;

#[test]
fn it_can_be_rotated_90_degrees_counter_clockwise() {
    assert_approx_eq!(Vec2D::new(2.0, 0.0).rotate_90(), &Vec2D::new(0.0, 2.0));
}
