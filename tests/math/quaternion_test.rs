use std::num::Float;

use math::Quaternion;

#[test]
fn new_test() {
    let q = Quaternion::new(0.3, -1.0, 0.1, 3.0);

    assert_eq!((q[0], q[1], q[2], q[3]), (0.3, -1.0, 0.1, 3.0));
}

#[test]
fn new_identity_test() {
    let q = Quaternion::new_identity();

    assert_eq!((q[0], q[1], q[2], q[3]), (1.0, 0.0, 0.0, 0.0));
}

#[test]
fn length_sq_test() {
    let q = Quaternion::new(1.0, 2.0, 3.0, -4.0);

    assert_eq!(q.length_sq(), 30.0);
}

#[test]
fn length_test() {
    let q = Quaternion::new(-3.0, 4.0, 2.0, -1.0);

    assert_eq!(q.length(), 30.0.sqrt());
}

#[test]
fn normalize_test() {
    let q = Quaternion::new(12.0, 0.0, -9.0, 20.0).normalize();

    assert_eq!((q[0], q[1], q[2], q[3]), (0.48, 0.0, -0.36, 0.80));
}

#[test]
fn inverse_test() {
    let q = Quaternion::new(1.0, 0.0, 1.0, 0.0).inverse();

    assert_eq!((q[0], q[1], q[2], q[3]), (0.5, 0.0, -0.5, 0.0));
}

#[test]
fn sub_test() {
    let q = Quaternion::new(1.0, 3.0, 4.0, -1.0).sub(1.0, 3.0, -4.0, 1.0);

    assert_eq!((q[0], q[1], q[2], q[3]), (0.0, 0.0, 8.0, -2.0));
}

#[test]
fn mult_test() {
    let q = Quaternion::new(3.0, 2.0, 1.0, -2.0).mult(-2.0, 2.0, -4.0, -3.0);

    assert_eq!((q[0], q[1], q[2], q[3]), (-12.0, -9.0, -12.0, -15.0));
}

#[cfg(test)]
mod impls {
    use math::Quaternion;

    #[test]
    fn clone_test() {
        let q = Quaternion::new(1.0, 3.0, 4.0, 5.0).clone();

        assert_eq!((q[0], q[1], q[2], q[3]), (1.0, 3.0, 4.0, 5.0));
    }

    #[test]
    fn index_setter_test() {
        let mut q = Quaternion::new(1.0, 3.0, 4.0, 5.0);

        q[0] = -1.1;
        q[2] = 3.0;

        assert_eq!((q[0], q[1], q[2], q[3]), (-1.1, 3.0, 3.0, 5.0));
    }

    #[test]
    fn negation_test() {
        let q = -Quaternion::new(2.0, -3.3, 4.5, -1.2);

        assert_eq!((q[0], q[1], q[2], q[3]), (-2.0, 3.3, -4.5, 1.2));
    }

    #[test]
    fn subtraction_test() {
        let q = Quaternion::new(1.0, 2.2, -2.6, -4.4) - Quaternion::new(1.0, -1.0, -2.6, -2.4);

        assert_eq!((q[0], q[1], q[2], q[3]), (0.0, 3.2, 0.0, -2.0));
    }

    #[test]
    fn quat_scalar_multiplication_test() {
        let q = Quaternion::new(3.0, 2.0, -1.0, 0.0) * 2.0;

        assert_eq!((q[0], q[1], q[2], q[3]), (6.0, 4.0, -2.0, 0.0));
    }

    #[test]
    fn quat_quat_multiplication_test() {
        let q = Quaternion::new(3.0, 2.0, 1.0, -2.0) * Quaternion::new(-2.0, 2.0, -4.0, -3.0);

        assert_eq!((q[0], q[1], q[2], q[3]), (-12.0, -9.0, -12.0, -15.0));
    }

    #[test]
    fn quat_scalar_division_test() {
        let q = Quaternion::new(3.0, 9.0, 15.0, -30.0) / 3.0;

        assert_eq!((q[0], q[1], q[2], q[3]), (1.0, 3.0, 5.0, -10.0));
    }
}
