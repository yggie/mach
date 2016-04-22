use maths::{UnitVec3D, Vec3D};

#[test]
fn computing_the_normalized_vector() {
    let n = Vec3D::new(12.0, -20.0, 9.0).normalize();

    assert_approx_eq!(n, UnitVec3D::from(Vec3D::new(0.48, -0.80, 0.36)));
}
