use mach::Float;
use mach::maths::{ Quaternion, Vector };

#[test]
fn instantiating_with_components() {
    let q = Quaternion::new(0.3, -1.0, 0.1, 3.0);

    assert_eq!((q[0], q[1], q[2], q[3]), (0.3, -1.0, 0.1, 3.0));
}

#[test]
fn instantiating_as_the_identity_rotation() {
    let q = Quaternion::new_identity();

    assert_eq!((q[0], q[1], q[2], q[3]), (1.0, 0.0, 0.0, 0.0));
}

#[test]
fn instantiating_from_a_vector() {
    let q = Quaternion::new_from_vector(Vector::new(0.1, 0.5, 0.6));

    assert_eq!((q[0], q[1], q[2], q[3]), (0.0, 0.1, 0.5, 0.6));
}

#[test]
fn instantiating_from_axis_angle() {
    let radians = 2.5;
    let hr = radians / 2.0;

    let q = Quaternion::new_from_axis_angle(Vector::new(2.0, 3.0, 6.0), radians);

    let chr = hr.cos();
    let shr = hr.sin();
    let diff = (q - Quaternion::new(chr, 2.0*shr/7.0, 3.0*shr/7.0, 6.0*shr/7.0)).length();
    assert!(diff < 0.001);
}

#[test]
fn computing_the_squared_length() {
    let q = Quaternion::new(1.0, 2.0, 3.0, -4.0);

    assert_eq!(q.length_sq(), 30.0);
}

#[test]
fn computing_the_length() {
    let q = Quaternion::new(-3.0, 4.0, 2.0, -1.0);

    assert_eq!(q.length(), (30.0 as Float).sqrt());
}

#[test]
fn computing_the_normalized_quaternion() {
    let q = Quaternion::new(12.0, 0.0, -9.0, 20.0).normalize();

    assert_eq!((q[0], q[1], q[2], q[3]), (0.48, 0.0, -0.36, 0.80));
}

#[test]
fn computing_the_inverse() {
    let q = Quaternion::new(1.0, 0.0, 1.0, 0.0).inverse();

    assert_eq!((q[0], q[1], q[2], q[3]), (0.5, 0.0, -0.5, 0.0));
}

#[test]
fn adding_with_scalar_components() {
    let q = Quaternion::new(1.0, 3.0, 4.0, -1.0).add(1.0, 3.0, -4.0, 1.0);

    assert_eq!((q[0], q[1], q[2], q[3]), (2.0, 6.0, 0.0, 0.0));
}

#[test]
fn subtracting_by_scalar_components() {
    let q = Quaternion::new(1.0, 3.0, 4.0, -1.0).sub(1.0, 3.0, -4.0, 1.0);

    assert_eq!((q[0], q[1], q[2], q[3]), (0.0, 0.0, 8.0, -2.0));
}

#[test]
fn subtracting_by_a_quaternion() {
    let q = Quaternion::new(1.0, 2.2, -2.6, -4.4) - Quaternion::new(1.0, -1.0, -2.6, -2.4);

    assert_eq!((q[0], q[1], q[2], q[3]), (0.0, 3.2, 0.0, -2.0));
}

#[test]
fn multiplication_with_scalar_components() {
    let q = Quaternion::new(3.0, 2.0, 1.0, -2.0).mult(-2.0, 2.0, -4.0, -3.0);

    assert_eq!((q[0], q[1], q[2], q[3]), (-12.0, -9.0, -12.0, -15.0));
}

#[test]
fn cloning() {
    let q = Quaternion::new(1.0, 3.0, 4.0, 5.0).clone();

    assert_eq!((q[0], q[1], q[2], q[3]), (1.0, 3.0, 4.0, 5.0));
}

#[test]
fn setting_by_index() {
    let mut q = Quaternion::new(1.0, 3.0, 4.0, 5.0);

    q[0] = -1.1;
    q[2] = 3.0;

    assert_eq!((q[0], q[1], q[2], q[3]), (-1.1, 3.0, 3.0, 5.0));
}

#[test]
fn negating() {
    let q = -Quaternion::new(2.0, -3.3, 4.5, -1.2);

    assert_eq!((q[0], q[1], q[2], q[3]), (-2.0, 3.3, -4.5, 1.2));
}

#[test]
fn multiplying_by_a_scalar() {
    let q = Quaternion::new(3.0, 2.0, -1.0, 0.0) * 2.0;

    assert_eq!((q[0], q[1], q[2], q[3]), (6.0, 4.0, -2.0, 0.0));
}

#[test]
fn multiplying_with_a_quaternion() {
    let q = Quaternion::new(3.0, 2.0, 1.0, -2.0) * Quaternion::new(-2.0, 2.0, -4.0, -3.0);

    assert_eq!((q[0], q[1], q[2], q[3]), (-12.0, -9.0, -12.0, -15.0));
}

#[test]
fn dividing_by_a_scalar() {
    let q = Quaternion::new(3.0, 9.0, 15.0, -30.0) / 3.0;

    assert_eq!((q[0], q[1], q[2], q[3]), (1.0, 3.0, 5.0, -10.0));
}
