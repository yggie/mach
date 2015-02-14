#[cfg(test)]
mod cube_cube {
    use std::num::Float;

    use core::{ Body, State };
    use math::{ PI, Vector };
    use space::Pair;
    use shapes::Cube;
    use materials::Rigid;

    fn setup_cubes(state_0: State, state_1: State) -> (Pair, [Body; 2]) {
        let shape = Cube::new(1.0, 1.0, 1.0);
        let material = Rigid::new(3.0);

        let body_0 = Body::new_with_id(0, Box::new(shape.clone()), Box::new(material), state_0);
        let body_1 = Body::new_with_id(1, Box::new(shape.clone()), Box::new(material), state_1);

        let pair = Pair::new(&body_0, &body_1);

        return (pair, [body_0, body_1]);
    }

    #[test]
    fn vertex_vertex_no_contact_test() {
        let (pair, bodies) = setup_cubes(
            State::new_stationary(),
            State::new_with_position(1.01, 1.01, 1.01),
        );

        let contact = pair.compute_contact(&bodies[0], &bodies[1]);

        assert!(contact.is_none());
    }

    #[test]
    fn vertex_vertex_contact_test() {
        let (pair, bodies) = setup_cubes(
            State::new_stationary(),
            State::new_with_position(0.99, 0.99, 0.99),
        );

        let contact = pair.compute_contact(&bodies[0], &bodies[1]);

        assert!(contact.is_some());
        // TODO officially support vertex – vertex contacts
        // let c = contact.unwrap();
        // assert!(c.point.distance_to(Vector::new(0.5, 0.0, 0.0)) < 0.001);
        // assert!(c.normal.distance_to(Vector::new(1.0, 0.0, 0.0)) < 0.001);
    }

    #[test]
    fn edge_edge_no_contact_test() {
        let (pair, bodies) = setup_cubes(
            State::new_stationary(),
            State::new_with_position(1.01, 1.01, 0.00),
        );

        let contact = pair.compute_contact(&bodies[0], &bodies[1]);

        assert!(contact.is_none());
    }

    #[test]
    fn edge_edge_contact_test() {
        let (pair, bodies) = setup_cubes(
            State::new_stationary(),
            State::new_with_position(0.99, 0.99, 0.00),
        );

        let contact = pair.compute_contact(&bodies[0], &bodies[1]);

        assert!(contact.is_some());
        // TODO officially support edge – edge contacts
        // let c = contact.unwrap();
        // assert!(c.point.distance_to(Vector::new(0.5, 0.0, 0.0)) < 0.001);
        // assert!(c.normal.distance_to(Vector::new(1.0, 0.0, 0.0)) < 0.001);
    }

    #[test]
    fn face_face_no_contact_test() {
        let (pair, bodies) = setup_cubes(
            State::new_stationary(),
            State::new_with_position(1.01, 0.5, 0.5),
        );

        let contact = pair.compute_contact(&bodies[0], &bodies[1]);

        assert!(contact.is_none());
    }

    #[test]
    fn face_face_contact_test() {
        let (pair, bodies) = setup_cubes(
            State::new_stationary(),
            State::new_with_position(0.99, 0.5, 0.5),
        );

        let contact = pair.compute_contact(&bodies[0], &bodies[1]);

        assert!(contact.is_some());
        // TODO officially support face – face contacts
        // let c = contact.unwrap();
        // assert!(c.point.distance_to(Vector::new(0.5, 0.0, 0.0)) < 0.001);
        // assert!(c.normal.distance_to(Vector::new(1.0, 0.0, 0.0)) < 0.001);
    }

    #[test]
    fn edge_face_no_contact_test() {
        let (pair, bodies) = setup_cubes(
            State::new_stationary(),
            State::new_with_position(0.51 + 0.5*2.0.sqrt(), 0.00, 0.00)
                .with_rotation(Vector::new(0.0, 1.0, 0.0), PI/4.0),
        );

        let contact = pair.compute_contact(&bodies[0], &bodies[1]);

        assert!(contact.is_none());
    }

    #[test]
    fn edge_face_contact_test() {
        let (pair, bodies) = setup_cubes(
            State::new_stationary(),
            State::new_with_position(0.49 + 0.5*2.0.sqrt(), 0.00, 0.00)
                .with_rotation(Vector::new(0.0, 1.0, 0.0), PI/4.0),
        );

        let contact = pair.compute_contact(&bodies[0], &bodies[1]);

        assert!(contact.is_some());
        // TODO officially support edge – face contacts
        // let c = contact.unwrap();
        // assert!(c.point.distance_to(Vector::new(0.5, 0.0, 0.0)) < 0.001);
        // assert!(c.normal.distance_to(Vector::new(1.0, 0.0, 0.0)) < 0.001);
    }

    #[test]
    fn vertex_face_no_contact_test() {
        // SETUP
        let initial_axis = Vector::new(1.0, 1.0, 1.0).normalize();
        let final_axis = Vector::new(1.0, 0.0, 0.0);
        let rotation = initial_axis.cross(final_axis);
        let state_1 = State::new_with_position((1.01 + 3.0.sqrt())/2.0, 0.0, 0.0)
            .with_rotation(rotation, rotation.length().asin());
        let (pair, bodies) = setup_cubes(State::new_stationary(), state_1);

        // EXERCISE
        let contact = pair.compute_contact(&bodies[0], &bodies[1]);

        // VERIFY
        assert!(contact.is_none());
    }

    #[test]
    fn vertex_face_contact_test() {
        // SETUP
        let initial_axis = Vector::new(1.0, 1.0, 1.0).normalize();
        let final_axis = Vector::new(1.0, 0.0, 0.0);
        let rotation = initial_axis.cross(final_axis);
        let state_1 = State::new_with_position((0.99 + 3.0.sqrt())/2.0, 0.0, 0.0)
            .with_rotation(rotation, rotation.length().asin());
        let (pair, bodies) = setup_cubes(State::new_stationary(), state_1);

        // EXERCISE
        let contact = pair.compute_contact(&bodies[0], &bodies[1]);

        for v in bodies[1].vertices_iter() {
            println!("{:?}", v);
        }

        // VERIFY
        assert!(contact.is_some());
        // TODO officially support vertex – face contacts
        // let c = contact.unwrap();
        // assert!(c.point.distance_to(Vector::new(0.5, 0.0, 0.0)) < 0.001);
        // assert!(c.normal.distance_to(Vector::new(1.0, 0.0, 0.0)) < 0.001);
    }
}
