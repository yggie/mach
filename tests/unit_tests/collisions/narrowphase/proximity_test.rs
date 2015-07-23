use mithril::core::{ Body, State };
use mithril::math::{ PI, Vector };
use mithril::shapes::Cube;
use mithril::materials::Rigid;
use mithril::collisions::Proximity;

fn setup_cubes(cube_0: Cube, state_0: State, cube_1: Cube, state_1: State) -> (Proximity<usize>, [Body<usize>; 2]) {
    let material = Rigid::new(3.0);

    let body_0 = Body::new_with_id(0, Box::new(cube_0), Box::new(material), state_0);
    let body_1 = Body::new_with_id(1, Box::new(cube_1), Box::new(material), state_1);

    let proximity = Proximity::new(0, 1);

    return (proximity, [body_0, body_1]);
}

#[test]
fn almost_colliding_vertex_to_vertex() {
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
fn colliding_vertex_to_vertex() {
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
fn almost_colliding_edge_to_edge() {
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
fn colliding_edge_to_edge() {
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
fn almost_colliding_face_to_face() {
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
fn colliding_face_to_face() {
    let (proximity, bodies) = setup_cubes(
        Cube::new(1.0, 1.0, 1.0),
        State::new_stationary(),
        Cube::new(1.0, 1.0, 1.0),
        State::new_with_position(0.99, 0.5, 0.5),
    );

    let contact = proximity.find_intersection(&bodies[0], &bodies[1]);

    assert!(contact.is_some());
    let (_, contact_normal) = contact.unwrap();
    assert_eq!(contact_normal, Vector::new(1.0, 0.0, 0.0));
    // TODO compute contact point
    // assert_eq!(c.point, Vector::new(0.995, 0.750, 0.750));
}

#[test]
fn almost_colliding_edge_to_face() {
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
fn colliding_edge_to_face() {
    let (proximity, bodies) = setup_cubes(
        Cube::new(1.0, 1.0, 1.0),
        State::new_stationary(),
        Cube::new(1.0, 1.0, 1.0),
        State::new_with_position(0.49 + 0.5*2.0f32.sqrt(), 0.00, 0.00)
            .with_rotation(Vector::new(0.0, 0.0, 1.0), PI/4.0),
    );

    let contact = proximity.find_intersection(&bodies[0], &bodies[1]);

    assert!(contact.is_some());
    let (_, contact_normal) = contact.unwrap();
    assert_eq!(contact_normal, Vector::new(1.0, 0.0, 0.0));
    // TODO compute contact point
    // assert_eq!(c.point, Vector::new(0.5, 0.0, 0.0));
}

#[test]
fn almost_colliding_vertex_to_face() {
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
fn colliding_vertex_to_face() {
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
    let (contact_center, contact_normal) = contact.unwrap();
    assert_eq!(contact_normal, Vector::new(1.0, 0.0, 0.0));
    assert_eq!(contact_center, Vector::new(0.495, 0.1, 0.0));
}
