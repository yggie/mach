use shapes::Sphere;

#[test]
fn new_test() {
    let s = Sphere::new(5.0);

    assert_eq!(s.radius, 5.0);
}

#[cfg(test)]
mod impls {
    use shapes::{ Shape, Sphere };
    use math::approx_eq;
    use std::f32::consts::PI;

    #[test]
    fn equality_test() {
        let a = Sphere::new(8.8);
        let b = Sphere::new(8.8);

        assert_eq!(a, b);
    }

    #[test]
    fn volume_test() {
        let s = Sphere::new(0.75);

        assert!(approx_eq(s.volume(), 0.5625*PI));
    }
}
