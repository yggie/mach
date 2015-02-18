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
    fn support_indices_for_test() {
        let c = Cube::new(2.0, 3.0, 1.0);
        let dir = Vector::new(-0.1, 1.0, 0.1);

        let indices = c.support_indices_for(dir);
        let v = c.vertex(indices[0]);

        assert_eq!(indices.len(), 1);
        assert_eq!((v[0], v[1], v[2]), (-1.0, 1.5, 0.5));

        let other_indices = c.support_indices_for(Vector::new(1.0, 0.0, 0.0));
        let other_vertices: Vec<Vector> = other_indices.iter()
            .map(|&i| c.vertex(i))
            .collect();

        assert_eq!(other_indices.len(), 4);
        let v0 = other_vertices[0];
        let v1 = other_vertices[1];
        let v2 = other_vertices[2];
        let v3 = other_vertices[3];
        assert_eq!((v0[0], v0[1], v0[2]), (1.0,  1.5,  0.5));
        assert_eq!((v1[0], v1[1], v1[2]), (1.0, -1.5,  0.5));
        assert_eq!((v2[0], v2[1], v2[2]), (1.0,  1.5, -0.5));
        assert_eq!((v3[0], v3[1], v3[2]), (1.0, -1.5, -0.5));
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
