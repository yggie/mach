use mach::core::Float;
use mach::maths::{ Vector, Matrix };

#[test]
fn instantiating_with_components() {
    let m = Matrix::new(1.0, 2.0, 3.0, 4.0, 5.0, 6.0, 7.0, 8.0, 9.0);

    assert_eq!((m[0], m[1], m[2]), (1.0, 2.0, 3.0));
    assert_eq!((m[3], m[4], m[5]), (4.0, 5.0, 6.0));
    assert_eq!((m[6], m[7], m[8]), (7.0, 8.0, 9.0));
}

#[test]
fn instantiating_from_a_slice() {
    let m = Matrix::new_from_slice(&[1.0, 2.0, 3.0, 4.0, 5.0, 6.0, 7.0, 8.0, 9.0]);

    assert_eq!((m[0], m[1], m[2]), (1.0, 2.0, 3.0));
    assert_eq!((m[3], m[4], m[5]), (4.0, 5.0, 6.0));
    assert_eq!((m[6], m[7], m[8]), (7.0, 8.0, 9.0));
}

#[test]
fn instantiating_as_the_identity_matrix() {
    let m = Matrix::new_identity();

    assert_eq!((m[0], m[1], m[2]), (1.0, 0.0, 0.0));
    assert_eq!((m[3], m[4], m[5]), (0.0, 1.0, 0.0));
    assert_eq!((m[6], m[7], m[8]), (0.0, 0.0, 1.0));
}

#[test]
fn instantiating_as_a_diagonal_matrix() {
    let m = Matrix::new_diag(1.0, 2.0, 3.0);

    assert_eq!((m[0], m[1], m[2]), (1.0, 0.0, 0.0));
    assert_eq!((m[3], m[4], m[5]), (0.0, 2.0, 0.0));
    assert_eq!((m[6], m[7], m[8]), (0.0, 0.0, 3.0));
}

#[test]
fn instantiating_as_a_skew_matrix() {
    let m = Matrix::new_skew(1.0, 2.0, 3.0);

    assert_eq!((m[0], m[1], m[2]), ( 0.0, -3.0,  2.0));
    assert_eq!((m[3], m[4], m[5]), ( 3.0,  0.0, -1.0));
    assert_eq!((m[6], m[7], m[8]), (-2.0,  1.0,  0.0));
}

#[test]
fn instantiating_from_axis_angle() {
    let a = Vector::new(0.0, 0.0, 1.0);
    let radians: Float = 3.0;
    let c = radians.cos();
    let s = radians.sin();
    let r = Matrix::new_rotation(radians, a);

    assert_eq!((r[0], r[1], r[2]), (  c,  -s, 0.0));
    assert_eq!((r[3], r[4], r[5]), (  s,   c, 0.0));
    assert_eq!((r[6], r[7], r[8]), (0.0, 0.0, 1.0));
}

#[test]
fn getting_elements_by_element_position() {
    let m = Matrix::new_diag(4.0, 5.0, 2.0);

    assert_eq!((m.get(0, 0), m.get(0, 1), m.get(0, 2)), (4.0, 0.0, 0.0));
    assert_eq!((m.get(1, 0), m.get(1, 1), m.get(1, 2)), (0.0, 5.0, 0.0));
    assert_eq!((m.get(2, 0), m.get(2, 1), m.get(2, 2)), (0.0, 0.0, 2.0));
}

#[test]
fn computing_the_determinant() {
    let matrix = Matrix::new(1.0, 2.0, 3.0, 4.0, 6.0, 5.0, 8.0, 7.0, 9.0);

    let determinant = matrix.determinant();

    assert_eq!(determinant, -33.0);
}

#[test]
fn computing_the_inverse() {
    let matrix = Matrix::new(1.0, 2.0, 3.0, 4.0, 5.0, 6.0, 8.0, 7.0, 9.0);

    let m = matrix.inverse();

        assert_eq!((m[0], m[1], m[2]), (-1.0/3.0, -1.0/3.0,  1.0/3.0));
        assert_eq!((m[3], m[4], m[5]), (-4.0/3.0,  5.0/3.0, -2.0/3.0));
        assert_eq!((m[6], m[7], m[8]), ( 4.0/3.0,     -1.0,  1.0/3.0));
}

#[test]
fn setting_by_index() {
    let mut m = Matrix::new(1.0, 2.0, 3.0, 4.0, 5.0, 6.0, 7.0, 8.0, 9.0);

    m[0] = 11.0;
    m[4] = 12.0;
    m[8] = 13.0;

    assert_eq!((m[0], m[1], m[2]), (11.0, 2.0, 3.0));
    assert_eq!((m[3], m[4], m[5]), (4.0, 12.0, 6.0));
    assert_eq!((m[6], m[7], m[8]), (7.0, 8.0, 13.0));
}

#[test]
fn negating() {
    let m = -Matrix::new(1.0, 2.0, 3.0, 4.0, 5.0, 6.0, 7.0, 8.0, 9.0);

    assert_eq!((m[0], m[1], m[2]), (-1.0, -2.0, -3.0));
    assert_eq!((m[3], m[4], m[5]), (-4.0, -5.0, -6.0));
    assert_eq!((m[6], m[7], m[8]), (-7.0, -8.0, -9.0));
}

#[test]
fn adding_a_matrix() {
    let a = Matrix::new(1.0, 2.0, 3.0, 4.0, 5.0, 6.0, 7.0, 8.0, 9.0);
    let b = Matrix::new_diag(3.0, 2.0, 1.0);

    let m = a + b;

    assert_eq!((m[0], m[1], m[2]), (4.0, 2.0, 3.0));
    assert_eq!((m[3], m[4], m[5]), (4.0, 7.0, 6.0));
    assert_eq!((m[6], m[7], m[8]), (7.0, 8.0, 10.0));
}

#[test]
fn subtrating_by_a_matrix() {
    let a = Matrix::new(1.0, 2.0, 3.0, 4.0, 5.0, 6.0, 7.0, 8.0, 9.0);
    let b = Matrix::new_diag(1.0, 2.0, 3.0);

    let m = a - b;

    assert_eq!((m[0], m[1], m[2]), (0.0, 2.0, 3.0));
    assert_eq!((m[3], m[4], m[5]), (4.0, 3.0, 6.0));
    assert_eq!((m[6], m[7], m[8]), (7.0, 8.0, 6.0));
}

#[test]
fn multiplying_by_a_scalar() {
    let matrix = Matrix::new(1.0, 2.0, 3.0, 4.0, 5.0, 6.0, 7.0, 8.0, 9.0);

    let m = matrix * 2.0;

    assert_eq!((m[0], m[1], m[2]), ( 2.0,  4.0,  6.0));
    assert_eq!((m[3], m[4], m[5]), ( 8.0, 10.0, 12.0));
    assert_eq!((m[6], m[7], m[8]), (14.0, 16.0, 18.0));
}

#[test]
fn dividing_by_a_scalar() {
    let matrix = Matrix::new(1.0, 2.0, 3.0, 4.0, 5.0, 6.0, 7.0, 8.0, 9.0);

    let m = matrix / 2.0;

    assert_eq!((m[0], m[1], m[2]), (0.5, 1.0, 1.5));
    assert_eq!((m[3], m[4], m[5]), (2.0, 2.5, 3.0));
    assert_eq!((m[6], m[7], m[8]), (3.5, 4.0, 4.5));
}

#[test]
fn multiplying_by_a_vector() {
    let m = Matrix::new(1.0, 2.0, 3.0, 4.0, 5.0, 6.0, 7.0, 8.0, 9.0);
    let v = Vector::new(1.0, 2.0, 3.0);

    let a: Vector = m * v;

    assert_eq!((a[0], a[1], a[2]), (30.0, 36.0, 42.0));
}

#[test]
fn multiplying_by_a_matrix() {
    let a = Matrix::new(1.0, 2.0, 3.0, 4.0, 5.0, 6.0, 7.0, 8.0, 9.0);
    let b = Matrix::new(3.0, 2.0, 1.0, 6.0, 5.0, 4.0, 9.0, 8.0, 7.0);

    let m = a * b;

    assert_eq!((m[0], m[1], m[2]), ( 42.0,  36.0,  30.0));
    assert_eq!((m[3], m[4], m[5]), ( 96.0,  81.0,  66.0));
    assert_eq!((m[6], m[7], m[8]), (150.0, 126.0, 102.0));
}
