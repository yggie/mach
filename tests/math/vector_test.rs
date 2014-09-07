use math::Vector;

#[test]
fn constructor_test() {
    let v = Vector::new(1.0, 2.0, 3.0);
    assert!(v.x == 1.0)
    assert!(v.y == 2.0)
    assert!(v.z == 3.0)
}
