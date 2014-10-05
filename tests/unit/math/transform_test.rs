use math::{ Vector, Matrix, Transform };

#[test]
fn rotation_test() {
    let r = Matrix::new_rotation(2.5, Vector::new(1.0, 0.0, 0.0));
    let transform = Transform::new_rotation(r);

    let m = transform.rotation_matrix();
    let v = transform.translation_vector();

    assert_eq!((m[0], m[1], m[2], v[0]), (r[0], r[1], r[2], 0.0));
    assert_eq!((m[3], m[4], m[5], v[1]), (r[3], r[4], r[5], 0.0));
    assert_eq!((m[6], m[7], m[8], v[2]), (r[6], r[7], r[8], 0.0));
}

#[test]
fn translation_test() {
    let t = Vector::new(1.0, 2.0, 3.0);
    let transform = Transform::new_translation(t);

    let m = transform.rotation_matrix();
    let v = transform.translation_vector();

    assert_eq!((m[0], m[1], m[2], v[0]), (1.0, 0.0, 0.0, 1.0));
    assert_eq!((m[3], m[4], m[5], v[1]), (0.0, 1.0, 0.0, 2.0));
    assert_eq!((m[6], m[7], m[8], v[2]), (0.0, 0.0, 1.0, 3.0));
}

#[test]
fn new_identity_test() {
    let t = Transform::new_identity();
    let m = t.rotation_matrix();
    let v = t.translation_vector();

    assert_eq!((m[0], m[1], m[2], v[0]), (1.0, 0.0, 0.0, 0.0));
    assert_eq!((m[3], m[4], m[5], v[1]), (0.0, 1.0, 0.0, 0.0));
    assert_eq!((m[6], m[7], m[8], v[2]), (0.0, 0.0, 1.0, 0.0));
}
