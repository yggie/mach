#[cfg(test)]
mod cube_cube {
    use core::{ Body, State, UID };
    use math::{ PI, Vector };
    use shapes::Cube;
    use materials::Rigid;
    use collisions::Proximity;

    fn setup_cubes(cube_0: Cube, state_0: State, cube_1: Cube, state_1: State) -> (Proximity<UID>, [Body<usize>; 2]) {
        let material = Rigid::new(3.0);

        let body_0 = Body::new_with_handle(0, Box::new(cube_0), Box::new(material), state_0);
        let body_1 = Body::new_with_handle(1, Box::new(cube_1), Box::new(material), state_1);

        let proximity = Proximity::new(0, 1);

        return (proximity, [body_0, body_1]);
    }

    #[test]
    fn vertex_vertex_no_contact_test() {
        let (proximity, bodies) = setup_cubes(
            Cube::new(1.0, 2.0, 1.0),
            State::new_stationary(),
            Cube::new(2.0, 1.0, 1.0),
            State::new_with_position(1.51, 1.51, 1.01),
        );

        let contact = proximity.find_intersection(&bodies[0], &bodies[1]);

        assert!(contact.is_none());
    }

    #[test]
    fn vertex_vertex_contact_test() {
        let (proximity, bodies) = setup_cubes(
            Cube::new(1.0, 2.0, 1.0),
            State::new_stationary(),
            Cube::new(2.0, 1.0, 1.0),
            State::new_with_position(1.49, 1.49, 0.99),
        );

        let contact = proximity.find_intersection(&bodies[0], &bodies[1]);

        assert!(contact.is_some());
        // TODO officially support vertex – vertex contacts
        // let c = contact.unwrap();
        // assert_eq!(c.normal, Vector::new(1.0, 0.0, 0.0));
        // assert_eq!(c.point, Vector::new(0.5, 0.0, 0.0));
    }

    #[test]
    fn edge_edge_no_contact_test() {
        let (proximity, bodies) = setup_cubes(
            Cube::new(1.0, 2.0, 3.0),
            State::new_stationary(),
            Cube::new(1.0, 2.0, 3.0),
            State::new_with_position(1.01, 1.51, 0.00),
        );

        let contact = proximity.find_intersection(&bodies[0], &bodies[1]);

        assert!(contact.is_none());
    }

    #[test]
    fn edge_edge_contact_test() {
        let (proximity, bodies) = setup_cubes(
            Cube::new(1.0, 2.0, 3.0),
            State::new_stationary(),
            Cube::new(1.0, 2.0, 3.0),
            State::new_with_position(0.99, 1.49, 0.00),
        );

        let contact = proximity.find_intersection(&bodies[0], &bodies[1]);

        assert!(contact.is_some());
        // TODO officially support edge – edge contacts
        // let c = contact.unwrap();
        // assert!(c.point.distance_to(Vector::new(0.5, 0.0, 0.0)) < 0.001);
        // assert!(c.normal.distance_to(Vector::new(1.0, 0.0, 0.0)) < 0.001);
    }

    #[test]
    fn face_face_no_contact_test() {
        let (proximity, bodies) = setup_cubes(
            Cube::new(1.0, 1.0, 1.0),
            State::new_stationary(),
            Cube::new(1.0, 1.0, 1.0),
            State::new_with_position(1.01, 0.5, 0.5),
        );

        let contact = proximity.find_intersection(&bodies[0], &bodies[1]);

        assert!(contact.is_none());
    }

    #[test]
    fn face_face_contact_test() {
        let (proximity, bodies) = setup_cubes(
            Cube::new(1.0, 1.0, 1.0),
            State::new_stationary(),
            Cube::new(1.0, 1.0, 1.0),
            State::new_with_position(0.99, 0.5, 0.5),
        );

        let contact = proximity.find_intersection(&bodies[0], &bodies[1]);

        assert!(contact.is_some());
        let c = contact.unwrap();
        assert_eq!(c.normal, Vector::new(1.0, 0.0, 0.0));
        // TODO compute contact point
        // assert_eq!(c.point, Vector::new(0.995, 0.750, 0.750));
    }

    #[test]
    fn edge_face_no_contact_test() {
        let (proximity, bodies) = setup_cubes(
            Cube::new(1.0, 1.0, 1.0),
            State::new_stationary(),
            Cube::new(1.0, 1.0, 1.0),
            State::new_with_position(0.51 + 0.5*2.0f32.sqrt(), 0.00, 0.00)
                .with_rotation(Vector::new(0.0, 1.0, 0.0), PI/4.0),
        );

        let contact = proximity.find_intersection(&bodies[0], &bodies[1]);

        assert!(contact.is_none());
    }

    #[test]
    fn edge_face_contact_test() {
        let (proximity, bodies) = setup_cubes(
            Cube::new(1.0, 1.0, 1.0),
            State::new_stationary(),
            Cube::new(1.0, 1.0, 1.0),
            State::new_with_position(0.49 + 0.5*2.0f32.sqrt(), 0.00, 0.00)
                .with_rotation(Vector::new(0.0, 0.0, 1.0), PI/4.0),
        );

        let contact = proximity.find_intersection(&bodies[0], &bodies[1]);

        assert!(contact.is_some());
        let c = contact.unwrap();
        assert_eq!(c.normal, Vector::new(1.0, 0.0, 0.0));
        // TODO compute contact point
        // assert_eq!(c.point, Vector::new(0.5, 0.0, 0.0));
    }

    #[test]
    fn vertex_face_no_contact_test() {
        // SETUP
        let initial_axis = Vector::new(1.0, 1.0, 1.0).normalize();
        let final_axis = Vector::new(1.0, 0.0, 0.0);
        let rotation = initial_axis.cross(final_axis);
        let state_1 = State::new_with_position((1.01 + 3.0f32.sqrt())/2.0, 0.0, 0.0)
            .with_rotation(rotation, rotation.length().asin());
        let (proximity, bodies) = setup_cubes(
            Cube::new(1.0, 1.0, 1.0),
            State::new_stationary(),
            Cube::new(1.0, 1.0, 1.0),
            state_1,
        );

        // EXERCISE
        let contact = proximity.find_intersection(&bodies[0], &bodies[1]);

        // VERIFY
        assert!(contact.is_none());
    }

    #[test]
    fn vertex_face_contact_test() {
        // SETUP
        let initial_axis = Vector::new(1.0, 1.0, 1.0).normalize();
        let final_axis = Vector::new(1.0, 0.0, 0.0);
        let rotation = initial_axis.cross(final_axis);
        let state_1 = State::new_with_position((0.98 + 3.0f32.sqrt())/2.0, 0.1, 0.0)
            .with_rotation(rotation, rotation.length().asin());
        let (proximity, bodies) = setup_cubes(
            Cube::new(1.0, 1.0, 1.0),
            State::new_stationary(),
            Cube::new(1.0, 1.0, 1.0),
            state_1,
        );

        // EXERCISE
        let contact = proximity.find_intersection(&bodies[0], &bodies[1]);

        // VERIFY
        assert!(contact.is_some());
        let c = contact.unwrap();
        assert_eq!(c.normal, Vector::new(1.0, 0.0, 0.0));
        assert_eq!(c.center, Vector::new(0.495, 0.1, 0.0));
    }
}
