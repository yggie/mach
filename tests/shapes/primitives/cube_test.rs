use shapes::Cube;

#[test]
fn new_test() {
    let c = Cube::new(5.0, 3.0, 7.5);

    assert_eq!((c.width, c.height, c.depth), (5.0, 3.0, 7.5));
}

#[cfg(test)]
mod impls {
    use shapes::{ Shape, Cube };

    #[test]
    fn equality_test() {
        let a = Cube::new(1.0, 2.0, 3.0);
        let b = Cube::new(1.0, 2.0, 3.0);

        assert_eq!(a, b);
    }

    #[test]
    fn volume_test() {
        let c = Cube::new(2.0, 3.0, 4.0);

        assert_eq!(c.volume(), 24.0);
    }
}
