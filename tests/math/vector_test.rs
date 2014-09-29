use math::Vector;

#[test]
fn new_test() {
    let v = Vector::new(1.0, 2.0, 3.0);

    assert_eq!((v[0], v[1], v[2]), (1.0, 2.0, 3.0));
}

#[test]
fn new_zero_test() {
    let v = Vector::new_zero();

    assert_eq!((v[0], v[1], v[2]), (0.0, 0.0, 0.0));
}

#[test]
fn scaling_test() {
    let a = Vector::new(1.0, 2.0, 3.0);
    let v = a.scale(2.5);

    assert_eq!((v[0], v[1], v[2]), (2.5, 5.0, 7.5));
}

#[test]
fn dot_product_test() {
    let a = Vector::new(1.0, 2.0, 3.0);
    let b = Vector::new(2.0, -2.0, 2.0);

    assert_eq!(a.dot(&b), 4.0);
}

#[test]
fn cross_product_test() {
    let a = Vector::new(1.0, 2.0, 1.0);
    let b = Vector::new(2.0, 1.0, 2.0);

    let c = a.cross(&b);

    assert_eq!((c[0], c[1], c[2]), (3.0, 0.0, -3.0));
}

#[test]
fn normalize_test() {
    let v = Vector::new(12.0, 20.0, 9.0);
    let n = v.normalize();

    assert_eq!((n[0], n[1], n[2]), (0.48, 0.80, 0.36));
}

#[test]
fn squared_length_test() {
    let v = Vector::new(1.0, -2.0, 3.0);

    assert_eq!(v.length_sq(), 14.0);
}

#[test]
fn length_test() {
    let v = Vector::new(2.0, 3.0, 6.0);

    assert_eq!(v.length(), 7.0);
}

#[test]
fn outer_product_test() {
    let a = Vector::new(1.0, 2.0, 3.0);
    let b = Vector::new(4.0, 5.0, 6.0);

    let m = a.outer(&b);

    assert_eq!((m[0], m[1], m[2]), ( 4.0,  5.0,  6.0));
    assert_eq!((m[3], m[4], m[5]), ( 8.0, 10.0, 12.0));
    assert_eq!((m[6], m[7], m[8]), (12.0, 15.0, 18.0));
}

#[cfg(test)]
mod impls {
    use math::Vector;

    #[test]
    fn index_test() {
        let v = Vector::new(1.0, 2.0, 3.0);

        assert_eq!((v[0], v[1], v[2]), (1.0, 2.0, 3.0));
    }

    #[test]
    fn index_mut_test() {
        let mut v = Vector::new(1.0, 2.0, 3.0);
        v[0] = 3.0;

        assert_eq!((v[0], v[1], v[2]), (3.0, 2.0, 3.0));
    }

    #[test]
    fn negation_test() {
        let a = Vector::new(1.0, 3.0, 9.0);
        let b = -a;

        assert_eq!((b[0], b[1], b[2]), (-1.0, -3.0, -9.0));
    }

    #[test]
    fn addition_test() {
        let a = Vector::new(1.0, 3.0, -1.0);
        let b = Vector::new(2.0, 1.0, 1.0);

        let c = a + b;

        assert_eq!((c[0], c[1], c[2]), (3.0, 4.0, 0.0));
    }

    #[test]
    fn subtraction_test() {
        let a = Vector::new(1.0, -1.0, 3.5);
        let b = Vector::new(1.0, 1.0, -3.5);

        let c = a - b;

        assert_eq!((c[0], c[1], c[2]), (0.0, -2.0, 7.0));
    }
}
