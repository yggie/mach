use Scalar;
use maths::{Quat, UnitQuat, Vect};

#[test]
fn it_can_be_instantiated_with_scalars() {
    let q = Quat::new(0.3, -1.0, 0.1, 3.0);

    assert_approx_eq!(q, Quat::new(0.3, -1.0, 0.1, 3.0));
}

#[test]
fn it_can_be_instantiated_as_the_identity_quaternion() {
    let q = Quat::identity();

    assert_approx_eq!(q, Quat::new(1.0, 0.0, 0.0, 0.0));
}

#[test]
fn it_can_be_instantiated_from_a_vector() {
    let q = Quat::from_vector(Vect::new(0.1, 0.5, 0.6));

    assert_approx_eq!(q, Quat::new(0.0, 0.1, 0.5, 0.6));
}

#[test]
fn it_can_compute_the_squared_length() {
    let q = Quat::new(1.0, 2.0, 3.0, -4.0);

    assert_approx_eq!(q.squared_length(), 30.0);
}

#[test]
fn it_can_compute_the_length() {
    let q = Quat::new(-3.0, 4.0, 2.0, -1.0);

    assert_approx_eq!(q.length(), (30.0 as Scalar).sqrt());
}

#[test]
fn it_can_be_normalized() {
    let q = Quat::new(12.0, 0.0, -9.0, 20.0);

    assert_approx_eq!(q.normalize(), UnitQuat::from_quat(Quat::new(0.48, 0.0, -0.36, 0.80)));
}

#[test]
fn it_can_compute_the_inverse() {
    let q = Quat::new(1.0, 0.0, 1.0, 0.0).inverse();

    assert_approx_eq!(q, Quat::new(0.5, 0.0, -0.5, 0.0));
}

#[test]
fn it_supports_the_subtraction_operator_with_quaternions() {
    let p = Quat::new(1.0, 2.2, -2.6, -4.4);
    let q = Quat::new(1.0, -1.0, -2.6, -2.4);

    let expectation = Quat::new(0.0, 3.2, 0.0, -2.0);

    assert_approx_eq!(p - q, expectation);
    assert_approx_eq!(&p - q, expectation);
    assert_approx_eq!(p - &q, expectation);
    assert_approx_eq!(&p - &q, expectation);
}

#[test]
fn it_can_be_cloned() {
    let q = Quat::new(1.0, 3.0, 4.0, 5.0);

    assert_approx_eq!(Clone::clone(&q), q);
}

#[test]
fn it_supports_the_unary_negation_operator() {
    let q = -Quat::new(2.0, -3.3, 4.5, -1.2);

    assert_approx_eq!(q, Quat::new(-2.0, 3.3, -4.5, 1.2));
}

#[test]
fn it_supports_quaternion_multiplication_with_a_scalar() {
    let q = Quat::new(3.0, 2.0, 1.0, -2.0);

    assert_approx_eq!(q.mult_scalar(-1.0), Quat::new(-3.0, -2.0, -1.0, 2.0));
}

#[test]
fn it_supports_scalar_multiplication_using_the_multiplication_operator() {
    let q = Quat::new(3.0, 2.0, -1.0, 0.0);

    let expectation = Quat::new(6.0, 4.0, -2.0, 0.0);

    assert_approx_eq!(q * 2.0, expectation);
    assert_approx_eq!(&q * 2.0, expectation);
}

#[test]
fn it_supports_quaternion_multiplication_with_scalars() {
    let q = Quat::new(3.0, 2.0, 1.0, -2.0);

    assert_approx_eq!(q.mult_quat(-2.0, 2.0, -4.0, -3.0), Quat::new(-12.0, -9.0, -12.0, -15.0));
}

#[test]
fn it_supports_quaternion_multiplication_using_the_multiplication_operator() {
    let a = Quat::new(3.0, 2.0, 1.0, -2.0);
    let b = Quat::new(-2.0, 2.0, -4.0, -3.0);

    let expectation = Quat::new(-12.0, -9.0, -12.0, -15.0);

    assert_approx_eq!(a * b, expectation);
    assert_approx_eq!(&a * b, expectation);
    assert_approx_eq!(a * &b, expectation);
    assert_approx_eq!(&a * &b, expectation);
}

#[test]
fn it_supports_the_division_operator_with_scalars() {
    let q = Quat::new(3.0, 9.0, 15.0, -30.0);

    let expectation = Quat::new(1.0, 3.0, 5.0, -10.0);

    assert_approx_eq!(q / 3.0, expectation);
    assert_approx_eq!(&q / 3.0, expectation);
}
