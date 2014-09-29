use math::{ Vector, Matrix };

#[test]
fn new_test() {
    let elems: [f32, ..9] = [1.0, 2.0, 3.0, 4.0, 5.0, 6.0, 7.0, 8.0, 9.0];
    let m = Matrix::new(&elems);

    assert_eq!((m[0], m[1], m[2]), (1.0, 2.0, 3.0));
    assert_eq!((m[3], m[4], m[5]), (4.0, 5.0, 6.0));
    assert_eq!((m[6], m[7], m[8]), (7.0, 8.0, 9.0));
}

#[test]
fn new_identity_test() {
    let m = Matrix::new_identity();

    assert_eq!((m[0], m[1], m[2]), (1.0, 0.0, 0.0));
    assert_eq!((m[3], m[4], m[5]), (0.0, 1.0, 0.0));
    assert_eq!((m[6], m[7], m[8]), (0.0, 0.0, 1.0));
}

#[test]
fn new_diag_test() {
    let m = Matrix::new_diag(1.0, 2.0, 3.0);

    assert_eq!((m[0], m[1], m[2]), (1.0, 0.0, 0.0));
    assert_eq!((m[3], m[4], m[5]), (0.0, 2.0, 0.0));
    assert_eq!((m[6], m[7], m[8]), (0.0, 0.0, 3.0));
}

#[test]
fn new_skew_test() {
    let m = Matrix::new_skew(1.0, 2.0, 3.0);

    assert_eq!((m[0], m[1], m[2]), ( 0.0, -3.0,  2.0));
    assert_eq!((m[3], m[4], m[5]), ( 3.0,  0.0, -1.0));
    assert_eq!((m[6], m[7], m[8]), (-2.0,  1.0,  0.0));
}

#[test]
fn new_rotation_test() {
    let a = Vector::new(0.0, 0.0, 1.0);
    let radians = 3.0f32;
    let c = radians.cos();
    let s = radians.sin();
    let r = Matrix::new_rotation(radians, a);

    assert_eq!((r[0], r[1], r[2]), (  c,  -s, 0.0));
    assert_eq!((r[3], r[4], r[5]), (  s,   c, 0.0));
    assert_eq!((r[6], r[7], r[8]), (0.0, 0.0, 1.0));
}

#[test]
fn element_getter_test() {
    let m = Matrix::new_diag(4.0, 5.0, 2.0);

    assert_eq!((m.get(0, 0), m.get(0, 1), m.get(0, 2)), (4.0, 0.0, 0.0));
    assert_eq!((m.get(1, 0), m.get(1, 1), m.get(1, 2)), (0.0, 5.0, 0.0));
    assert_eq!((m.get(2, 0), m.get(2, 1), m.get(2, 2)), (0.0, 0.0, 2.0));
}

#[test]
fn matrix_multiplication_test() {
    let elems_a: [f32, ..9] = [1.0, 2.0, 3.0, 4.0, 5.0, 6.0, 7.0, 8.0, 9.0];
    let a = Matrix::new(&elems_a);
    let elems_b: [f32, ..9] = [3.0, 2.0, 1.0, 6.0, 5.0, 4.0, 9.0, 8.0, 7.0];
    let b = Matrix::new(&elems_b);

    let m = Matrix::mult(&a, &b);

    assert_eq!((m[0], m[1], m[2]), ( 42.0,  36.0,  30.0));
    assert_eq!((m[3], m[4], m[5]), ( 96.0,  81.0,  66.0));
    assert_eq!((m[6], m[7], m[8]), (150.0, 126.0, 102.0));
}

#[cfg(test)]
mod impls {
    use math::{ Matrix, Vector };

    #[test]
    fn index_getter_test() {
        let elems: [f32, ..9] = [1.0, 2.0, 3.0, 4.0, 5.0, 6.0, 7.0, 8.0, 9.0];
        let m = Matrix::new(&elems);

        assert_eq!((m[0], m[1], m[2]), (1.0, 2.0, 3.0));
        assert_eq!((m[3], m[4], m[5]), (4.0, 5.0, 6.0));
        assert_eq!((m[6], m[7], m[8]), (7.0, 8.0, 9.0));
    }

    #[test]
    fn index_setter_test() {
        let elems: [f32, ..9] = [1.0, 2.0, 3.0, 4.0, 5.0, 6.0, 7.0, 8.0, 9.0];
        let mut m = Matrix::new(&elems);

        m[0] = 11.0;
        m[4] = 12.0;
        m[8] = 13.0;

        assert_eq!((m[0], m[1], m[2]), (11.0, 2.0, 3.0));
        assert_eq!((m[3], m[4], m[5]), (4.0, 12.0, 6.0));
        assert_eq!((m[6], m[7], m[8]), (7.0, 8.0, 13.0));
    }

    #[test]
    fn negation_test() {
        let elems: [f32, ..9] = [1.0, 2.0, 3.0, 4.0, 5.0, 6.0, 7.0, 8.0, 9.0];
        let m = -Matrix::new(&elems);

        assert_eq!((m[0], m[1], m[2]), (-1.0, -2.0, -3.0));
        assert_eq!((m[3], m[4], m[5]), (-4.0, -5.0, -6.0));
        assert_eq!((m[6], m[7], m[8]), (-7.0, -8.0, -9.0));
    }

    #[test]
    fn addition_test() {
        let elems: [f32, ..9] = [1.0, 2.0, 3.0, 4.0, 5.0, 6.0, 7.0, 8.0, 9.0];
        let a = Matrix::new(&elems);
        let b = Matrix::new_diag(3.0, 2.0, 1.0);

        let m = a + b;

        assert_eq!((m[0], m[1], m[2]), (4.0, 2.0, 3.0));
        assert_eq!((m[3], m[4], m[5]), (4.0, 7.0, 6.0));
        assert_eq!((m[6], m[7], m[8]), (7.0, 8.0, 10.0));
    }

    #[test]
    fn subtraction_test() {
        let elems: [f32, ..9] = [1.0, 2.0, 3.0, 4.0, 5.0, 6.0, 7.0, 8.0, 9.0];
        let a = Matrix::new(&elems);
        let b = Matrix::new_diag(1.0, 2.0, 3.0);

        let m = a - b;

        assert_eq!((m[0], m[1], m[2]), (0.0, 2.0, 3.0));
        assert_eq!((m[3], m[4], m[5]), (4.0, 3.0, 6.0));
        assert_eq!((m[6], m[7], m[8]), (7.0, 8.0, 6.0));
    }

    #[test]
    fn vector_multiplication_test() {
        let elems: [f32, ..9] = [1.0, 2.0, 3.0, 4.0, 5.0, 6.0, 7.0, 8.0, 9.0];
        let m = Matrix::new(&elems);
        let v = Vector::new(1.0, 2.0, 3.0);

        let a = m * v;

        assert_eq!((a[0], a[1], a[2]), (30.0, 36.0, 42.0));
    }
}
