use mach::core::PI;
use mach::maths::{ Vector, Quaternion };

#[test]
fn instantiating_with_components() {
    let v = Vector::new(1.0, 2.0, 3.0);

    assert_eq!((v[0], v[1], v[2]), (1.0, 2.0, 3.0));
}

#[test]
fn instantiating_as_a_zero_vector() {
    let v = Vector::new_zero();

    assert_eq!((v[0], v[1], v[2]), (0.0, 0.0, 0.0));
}

#[test]
fn adding_with_scalar_components() {
    let v = Vector::new(1.0, 2.0, 3.0).add(-1.0, 2.0, -3.0);

    assert_eq!((v[0], v[1], v[2]), (0.0, 4.0, 0.0));
}

#[test]
fn adding_a_vector() {
    let a = Vector::new(1.0, 3.0, -1.0);
    let b = Vector::new(2.0, 1.0, 1.0);

    let c = a + b;

    assert_eq!((c[0], c[1], c[2]), (3.0, 4.0, 0.0));
}

#[test]
fn subtracting_by_scalar_components() {
    let v = Vector::new(4.0, 5.0, 4.5).sub(4.0, 4.0, 5.0);

    assert_eq!((v[0], v[1], v[2]), (0.0, 1.0, -0.5));
}

#[test]
fn subtracting_by_a_vector() {
    let a = Vector::new(1.0, -1.0, 3.5);
    let b = Vector::new(1.0, 1.0, -3.5);

    let c = a - b;

    assert_eq!((c[0], c[1], c[2]), (0.0, -2.0, 7.0));
}

#[test]
fn computing_the_inner_product() {
    let a = Vector::new(1.0, 2.0, 3.0);
    let b = Vector::new(2.0, -2.0, 2.0);

    assert_eq!(a.dot(b), 4.0);
}

#[test]
fn computing_the_cross_product() {
    let v = Vector::new(1.0, 2.0, 1.0).cross(Vector::new(2.0, 1.0, 2.0));

    assert_eq!((v[0], v[1], v[2]), (3.0, 0.0, -3.0));
}

#[test]
fn computing_the_outer_product() {
    let a = Vector::new(1.0, 2.0, 3.0);
    let b = Vector::new(4.0, 5.0, 6.0);

    let m = a.outer(b);

    assert_eq!((m[0], m[1], m[2]), ( 4.0,  5.0,  6.0));
    assert_eq!((m[3], m[4], m[5]), ( 8.0, 10.0, 12.0));
    assert_eq!((m[6], m[7], m[8]), (12.0, 15.0, 18.0));
}

#[test]
fn computing_the_normalized_vector() {
    let n = Vector::new(12.0, -20.0, 9.0).normalize();

    assert_eq!((n[0], n[1], n[2]), (0.48, -0.80, 0.36));
}

#[test]
fn computing_the_squared_length() {
    let v = Vector::new(1.0, -2.0, 3.0);

    assert_eq!(v.length_sq(), 14.0);
}

#[test]
fn computing_the_vector_length() {
    let v = Vector::new(2.0, 3.0, 6.0);

    assert_eq!(v.length(), 7.0);
}

#[test]
fn computing_distance_between_vectors() {
    let a = Vector::new(1.0, 5.0, 2.0);
    let b = Vector::new(1.0, 2.0, -2.0);

    let d = a.distance_to(b);

    assert_eq!(d, 5.0);
}

#[test]
fn rotating_by_a_quaternion() {
    let v = Vector::new(1.0, 0.0, 0.0);
    let q = Quaternion::new_from_axis_angle(Vector::new(1.0, 0.5, 0.5), PI/3.0);

    let res = v.rotate_by_quaternion(q);

    assert!(res.distance_to(Vector::new(0.8333333333333335, 0.5202200572599405, -0.18688672392660716)) < 0.001);
}

#[test]
fn cloning() {
    let v = Vector::new(1.0, 2.0, 3.0).clone();

    assert_eq!((v[0], v[1], v[2]), (1.0, 2.0, 3.0));
}

#[test]
fn dereferencing_elements_by_index() {
    let v = Vector::new(1.0, 2.0, 3.0);

    assert_eq!((v[0], v[1], v[2]), (1.0, 2.0, 3.0));
}

#[test]
fn mutably_dereferencing_elements_by_index() {
    let mut v = Vector::new(1.0, 2.0, 3.0);
    v[0] = 3.0;

    assert_eq!((v[0], v[1], v[2]), (3.0, 2.0, 3.0));
}

#[test]
fn negating() {
    let a = Vector::new(1.0, 3.0, 9.0);
    let b = -a;

    assert_eq!((b[0], b[1], b[2]), (-1.0, -3.0, -9.0));
}

#[test]
fn multiplying_by_a_scalar() {
    let v = Vector::new(1.0, 2.0, 3.0) * 2.5;

    assert_eq!((v[0], v[1], v[2]), (2.5, 5.0, 7.5));
}
