use mach::{Scalar, PI};
use mach::maths::{Vect, Quat};

#[test]
fn it_can_be_instantiated_with_scalars() {
    let v = Vect::new(1.0, 2.0, 3.0);

    assert_approx_eq!(v, Vect::new(1.0, 2.0, 3.0));
}

#[test]
fn it_can_be_instatiated_as_the_zero_vector() {
    let v = Vect::zero();

    assert_approx_eq!(v, Vect::new(0.0, 0.0, 0.0));
}

#[test]
fn it_can_be_cloned() {
    let v = Vect::new(2.0, -1.0, 5.0);

    assert_approx_eq!(Clone::clone(&v), v);
}

#[test]
fn it_can_be_set_with_scalars() {
    let mut v = Vect::new(0.0, 0.0, 0.0);

    v.set(&(1.0, 2.0, 3.0));

    assert_approx_eq!(v, Vect::new(1.0, 2.0, 3.0));
}

#[test]
fn it_can_be_set_to_a_vector() {
    let mut v = Vect::new(0.0, 0.0, 0.0);
    let u = Vect::new(1.0, -3.0, 4.0);

    v.set(&u);

    assert_approx_eq!(v, u);
}

#[test]
fn it_can_add_scalar_components() {
    let v = Vect::new(1.0, 2.0, 3.0).add(-1.0, 2.0, -3.0);

    assert_approx_eq!(v, Vect::new(0.0, 4.0, 0.0));
}

#[test]
fn it_supports_the_addition_operator_with_vectors() {
    let a = Vect::new(1.0, 3.0, -1.0);
    let b = Vect::new(2.0, 1.0, 1.0);

    let expectation = Vect::new(3.0, 4.0, 0.0);

    assert_approx_eq!(a + b, expectation);
    assert_approx_eq!(&a + b, expectation);
    assert_approx_eq!(a + &b, expectation);
    assert_approx_eq!(&a + &b, expectation);
}

#[test]
fn it_can_subtract_scalar_components() {
    let v = Vect::new(4.0, 5.0, 4.5).sub(4.0, 4.0, 5.0);

    assert_approx_eq!(v, Vect::new(0.0, 1.0, -0.5));
}

#[test]
fn it_supports_the_subtraction_operator_with_vectors() {
    let a = Vect::new(1.0, -1.0, 3.5);
    let b = Vect::new(1.0, 1.0, -3.5);

    let expectation = Vect::new(0.0, -2.0, 7.0);

    assert_approx_eq!(a - b, expectation);
    assert_approx_eq!(&a - b, expectation);
    assert_approx_eq!(a - &b, expectation);
    assert_approx_eq!(&a - &b, expectation);
}

#[test]
fn it_can_dereference_to_an_array() {
    let v = Vect::new(1.0, 2.0, 3.0);
    let array: &[Scalar; 3] = v.as_ref();

    assert_eq!((array[0], array[1], array[2]), (1.0, 2.0, 3.0));
}

#[test]
fn it_can_dereference_to_a_tuple() {
    let v = Vect::new(1.0, 2.0, 3.0);
    let tuple: &(Scalar, Scalar, Scalar) = v.as_ref();

    assert_eq!((tuple.0, tuple.1, tuple.2), (1.0, 2.0, 3.0));
}

#[test]
fn computing_the_inner_product() {
    let a = Vect::new(1.0, 2.0, 3.0);
    let b = Vect::new(2.0, -2.0, 2.0);

    assert_approx_eq!(a.dot(b), 4.0);
}

#[test]
fn computing_the_cross_product() {
    let v = Vect::new(1.0, 2.0, 1.0).cross(Vect::new(2.0, 1.0, 2.0));

    assert_approx_eq!(v, Vect::new(3.0, 0.0, -3.0));
}

#[test]
fn computing_the_outer_product() {
    let a = Vect::new(1.0, 2.0, 3.0);
    let b = Vect::new(4.0, 5.0, 6.0);

    let m = a.outer(b);

    assert_approx_eq!(Vect::new(m[0], m[1], m[2]), Vect::new( 4.0,  5.0,  6.0));
    assert_approx_eq!(Vect::new(m[3], m[4], m[5]), Vect::new( 8.0, 10.0, 12.0));
    assert_approx_eq!(Vect::new(m[6], m[7], m[8]), Vect::new(12.0, 15.0, 18.0));
}

#[test]
fn computing_the_normalized_vector() {
    let n = Vect::new(12.0, -20.0, 9.0).normalize();

    assert_approx_eq!(n, Vect::new(0.48, -0.80, 0.36));
}

#[test]
fn computing_the_squared_length() {
    let v = Vect::new(1.0, -2.0, 3.0);

    assert_approx_eq!(v.length_sq(), 14.0);
}

#[test]
fn computing_the_vector_length() {
    let v = Vect::new(2.0, 3.0, 6.0);

    assert_approx_eq!(v.length(), 7.0);
}

#[test]
fn computing_distance_between_vectors() {
    let a = Vect::new(1.0, 5.0, 2.0);
    let b = Vect::new(1.0, 2.0, -2.0);

    let d = a.distance_to(b);

    assert_approx_eq!(d, 5.0);
}

#[test]
fn rotating_by_a_quaternion() {
    let v = Vect::new(1.0, 0.0, 0.0);
    let q = Quat::from_axis_angle(Vect::new(1.0, 0.5, 0.5), PI/3.0);

    let res = v.rotate_by_quaternion(q);

    assert_approx_eq!(res, Vect::new(0.8333333333333335, 0.5202200572599405, -0.18688672392660716));
}

#[test]
fn cloning() {
    let v = Vect::new(1.0, 2.0, 3.0).clone();

    assert_approx_eq!(v, Vect::new(1.0, 2.0, 3.0));
}

#[test]
fn dereferencing_elements_by_index() {
    let v = Vect::new(1.0, 2.0, 3.0);

    assert_approx_eq!(v, Vect::new(1.0, 2.0, 3.0));
}

#[test]
fn mutably_dereferencing_elements_by_index() {
    let mut v = Vect::new(1.0, 2.0, 3.0);
    v.x = 3.0;

    assert_approx_eq!(v, Vect::new(3.0, 2.0, 3.0));
}

#[test]
fn negating() {
    let a = Vect::new(1.0, 3.0, 9.0);
    let b = -a;

    assert_approx_eq!(b, Vect::new(-1.0, -3.0, -9.0));
}

#[test]
fn multiplying_by_a_scalar() {
    let v = Vect::new(1.0, 2.0, 3.0);
    let expected = Vect::new(2.5, 5.0, 7.5);

    assert_approx_eq!(v * 2.5, expected);
    assert_approx_eq!(2.5 * v, expected);
    assert_approx_eq!(&v * 2.5, expected);
    assert_approx_eq!(2.5 * &v, expected);

    assert_approx_eq!(v * 2.5, expected);
}