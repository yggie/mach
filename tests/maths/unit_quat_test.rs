use PI;
use maths::{Quat, UnitQuat, Vec3D};

#[test]
fn it_can_be_instantiated_using_the_axis_angle_formulation() {
    let radians = 2.5;
    let hr = radians / 2.0;

    let q = UnitQuat::from_axis_angle(Vec3D::new(2.0, 3.0, 6.0), radians);

    let chr = hr.cos();
    let shr = hr.sin();
    assert_approx_eq!(Quat::from(q), Quat::new(chr, 2.0*shr/7.0, 3.0*shr/7.0, 6.0*shr/7.0));
}

#[test]
fn it_can_rotate_a_vector() {
    let v = Vec3D::new(1.0, 0.0, 0.0);
    let q = UnitQuat::from_axis_angle(Vec3D::new(1.0, 0.5, 0.5), PI/3.0);

    let res = q.rotate(v);

    assert_approx_eq!(res, Vec3D::new(0.8333333333333335, 0.5202200572599405, -0.18688672392660716));
}
