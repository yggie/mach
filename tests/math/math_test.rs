use math::approx_eq;

#[test]
fn approx_eq_test() {
    assert!(approx_eq(1.000001, 1.0000015))
}
