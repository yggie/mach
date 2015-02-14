use shapes::Sphere;

#[test]
fn new_test() {
    let s = Sphere::new(5.0);

    assert_eq!(s.radius(), 5.0);
}

#[cfg(test)]
mod impls {
    use std::f32::consts::PI;

    use math::{ approx_eq, Vector };
    use shapes::{ Shape, Sphere };

    #[test]
    fn vertex_test() {
        let s = Sphere::new(3.3);

        let v = s.vertex(0);
        assert_eq!((v[0], v[1], v[2]), (0.0, 0.0, 0.0));
    }

    #[test]
    fn support_index_for_test() {
        let s = Sphere::new(5.0);

        assert_eq!(s.support_index_for(Vector::new(1.0, 1.0, 1.0)), 0);
    }


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
