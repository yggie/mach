use Scalar;
use maths::{DotProduct, Vec3D};

#[test]
fn it_can_be_instantiated_with_scalars() {
    let v = Vec3D::new(1.0, 2.0, 3.0);

    assert_approx_eq!(v, Vec3D::new(1.0, 2.0, 3.0));
}

#[test]
fn it_can_be_instatiated_as_the_zero_vector() {
    let v = Vec3D::zero();

    assert_approx_eq!(v, Vec3D::new(0.0, 0.0, 0.0));
}

#[test]
fn it_can_be_cloned() {
    let v = Vec3D::new(2.0, -1.0, 5.0);

    assert_approx_eq!(Clone::clone(&v), v);
}

#[test]
fn it_can_be_set_with_scalars() {
    let mut v = Vec3D::new(0.0, 0.0, 0.0);

    v.set(&(1.0, 2.0, 3.0));

    assert_approx_eq!(v, Vec3D::new(1.0, 2.0, 3.0));
}

#[test]
fn it_can_be_set_to_a_vector() {
    let mut v = Vec3D::new(0.0, 0.0, 0.0);
    let u = Vec3D::new(1.0, -3.0, 4.0);

    v.set(&u);

    assert_approx_eq!(v, u);
}

#[test]
fn it_supports_the_addition_operator_with_vectors() {
    let a = Vec3D::new(1.0, 3.0, -1.0);
    let b = Vec3D::new(2.0, 1.0, 1.0);

    let expectation = Vec3D::new(3.0, 4.0, 0.0);

    assert_approx_eq!(a + b, expectation);
    assert_approx_eq!(&a + b, expectation);
    assert_approx_eq!(a + &b, expectation);
    assert_approx_eq!(&a + &b, expectation);
}

#[test]
fn it_supports_the_subtraction_operator_with_vectors() {
    let a = Vec3D::new(1.0, -1.0, 3.5);
    let b = Vec3D::new(1.0, 1.0, -3.5);

    let expectation = Vec3D::new(0.0, -2.0, 7.0);

    assert_approx_eq!(a - b, expectation);
    assert_approx_eq!(&a - b, expectation);
    assert_approx_eq!(a - &b, expectation);
    assert_approx_eq!(&a - &b, expectation);
}

#[test]
fn it_can_dereference_to_an_array() {
    let v = Vec3D::new(1.0, 2.0, 3.0);
    let array: &[Scalar; 3] = v.as_ref();

    assert_eq!((array[0], array[1], array[2]), (1.0, 2.0, 3.0));
}

#[test]
fn it_can_dereference_to_a_tuple() {
    let v = Vec3D::new(1.0, 2.0, 3.0);
    let tuple: &(Scalar, Scalar, Scalar) = v.as_ref();

    assert_eq!((tuple.0, tuple.1, tuple.2), (1.0, 2.0, 3.0));
}

#[test]
fn computing_the_inner_product() {
    let a = Vec3D::new(1.0, 2.0, 3.0);
    let b = Vec3D::new(2.0, -2.0, 2.0);

    assert_approx_eq!(a.dot(b), 4.0);
}

#[test]
fn computing_the_cross_product() {
    let v = Vec3D::new(1.0, 2.0, 1.0).cross(Vec3D::new(2.0, 1.0, 2.0));

    assert_approx_eq!(v, Vec3D::new(3.0, 0.0, -3.0));
}

#[test]
fn computing_the_outer_product() {
    let a = Vec3D::new(1.0, 2.0, 3.0);
    let b = Vec3D::new(4.0, 5.0, 6.0);

    let m = a.outer(b);

    assert_approx_eq!(Vec3D::new(m[0], m[1], m[2]), Vec3D::new( 4.0,  5.0,  6.0));
    assert_approx_eq!(Vec3D::new(m[3], m[4], m[5]), Vec3D::new( 8.0, 10.0, 12.0));
    assert_approx_eq!(Vec3D::new(m[6], m[7], m[8]), Vec3D::new(12.0, 15.0, 18.0));
}

#[test]
fn computing_the_normalized_vector() {
    let n = Vec3D::new(12.0, -20.0, 9.0).normalize();

    assert_approx_eq!(n, Vec3D::new(0.48, -0.80, 0.36));
}

#[test]
fn computing_the_squared_length() {
    let v = Vec3D::new(1.0, -2.0, 3.0);

    assert_approx_eq!(v.length_sq(), 14.0);
}

#[test]
fn computing_the_vector_length() {
    let v = Vec3D::new(2.0, 3.0, 6.0);

    assert_approx_eq!(v.length(), 7.0);
}

#[test]
fn computing_distance_between_vectors() {
    let a = Vec3D::new(1.0, 5.0, 2.0);
    let b = Vec3D::new(1.0, 2.0, -2.0);

    let d = a.distance_to(b);

    assert_approx_eq!(d, 5.0);
}

#[test]
fn cloning() {
    let v = Vec3D::new(1.0, 2.0, 3.0).clone();

    assert_approx_eq!(v, Vec3D::new(1.0, 2.0, 3.0));
}

#[test]
fn dereferencing_elements_by_index() {
    let v = Vec3D::new(1.0, 2.0, 3.0);

    assert_approx_eq!(v, Vec3D::new(1.0, 2.0, 3.0));
}

#[test]
fn mutably_dereferencing_elements_by_index() {
    let mut v = Vec3D::new(1.0, 2.0, 3.0);
    v.x = 3.0;

    assert_approx_eq!(v, Vec3D::new(3.0, 2.0, 3.0));
}

#[test]
fn negating() {
    let a = Vec3D::new(1.0, 3.0, 9.0);
    let b = -a;

    assert_approx_eq!(b, Vec3D::new(-1.0, -3.0, -9.0));
}

#[test]
fn multiplying_by_a_scalar() {
    let v = Vec3D::new(1.0, 2.0, 3.0);
    let expected = Vec3D::new(2.5, 5.0, 7.5);

    assert_approx_eq!(v * 2.5, expected);
    assert_approx_eq!(2.5 * v, expected);
    assert_approx_eq!(&v * 2.5, expected);
    assert_approx_eq!(2.5 * &v, expected);

    assert_approx_eq!(v * 2.5, expected);
}
