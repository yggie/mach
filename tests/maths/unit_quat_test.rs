use PI;
use maths::{Quat, UnitQuat, Vect};

#[test]
fn it_can_be_instantiated_using_the_axis_angle_formulation() {
    let radians = 2.5;
    let hr = radians / 2.0;

    let q = UnitQuat::from_axis_angle(Vect::new(2.0, 3.0, 6.0), radians);

    let chr = hr.cos();
    let shr = hr.sin();
    assert_approx_eq!(q.to_quat(), Quat::new(chr, 2.0*shr/7.0, 3.0*shr/7.0, 6.0*shr/7.0));
}

#[test]
fn it_can_rotate_a_vector() {
    let v = Vect::new(1.0, 0.0, 0.0);
    let q = UnitQuat::from_axis_angle(Vect::new(1.0, 0.5, 0.5), PI/3.0);

    let res = q.rotate(v);

    assert_approx_eq!(res, Vect::new(0.8333333333333335, 0.5202200572599405, -0.18688672392660716));
}
