use properties::Rigid;

#[test]
fn new_test() {
    let r = Rigid::new(1.5);

    assert_eq!(r.density, 1.5)
}

#[cfg(test)]
mod impls {
    use shapes::Cube;
    use properties::{ Property, Rigid };

    #[test]
    fn equality_test() {
        let a = Rigid::new(1.0);
        let b = Rigid::new(1.0);

        assert_eq!(a, b)
    }

    #[test]
    fn mass_of_test() {
        let c = Cube::new(1.0, 2.0, 3.0);
        let p = Rigid::new(2.0);

        assert_eq!(p.mass_of(&c), 12.0)
    }

    #[test]
    fn density_of_test() {
        let c = Cube::new(1.0, 2.0, 3.0);
        let p = Rigid::new(1.5);

        assert_eq!(p.density_of(&c), 1.5)
    }
}
