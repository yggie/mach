use shapes::Cube;

#[test]
fn new_test() {
    let c = Cube::new(5.0, 3.0, 7.5);

    assert_eq!((c.width, c.height, c.depth), (5.0, 3.0, 7.5));
}

#[cfg(test)]
mod impls {
    use math::Vector;
    use shapes::{ Shape, Cube };

    #[test]
    fn vertices_len_test() {
        let c = Cube::new(3.0, 2.0, 1.0);

        assert_eq!(c.vertices_len(), 8);
    }

    #[test]
    fn support_index_for_test() {
        let c = Cube::new(2.0, 3.0, 1.0);
        let dir = Vector::new(-0.1, 1.0, 0.1);

        let index = c.support_index_for(dir);
        let v = c.vertex(index);

        assert_eq!((v[0], v[1], v[2]), (-1.0, 1.5, 0.5));
    }

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
