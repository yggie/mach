use mach::core::State;
use mach::maths::{ PI, Vector };
use mach::shapes::Cube;
use mach::entities::{ Material, RigidBody };
use mach::collisions::narrowphase::GjkEpaImplementation;

fn setup_cubes(cube_0: Cube, state_0: State, cube_1: Cube, state_1: State) -> (GjkEpaImplementation, [RigidBody; 2]) {
    let material = &Material::new_with_density(3.0);

    let body_0 = RigidBody::new_with_id(0, Box::new(cube_0), material, state_0);
    let body_1 = RigidBody::new_with_id(1, Box::new(cube_1), material, state_1);

    return (GjkEpaImplementation, [body_0, body_1]);
}

#[test]
fn almost_colliding_vertex_to_vertex() {
    let (narrowphase, bodies) = setup_cubes(
        Cube::new(1.0, 2.0, 1.0),
        State::new_stationary(),
        Cube::new(2.0, 1.0, 1.0),
        State::new_with_position(1.51, 1.51, 1.01),
    );

    let intersection = narrowphase.find_intersection(&bodies[0], &bodies[1]);

    assert!(intersection.is_none());
}

#[test]
fn colliding_vertex_to_vertex() {
    let (narrowphase, bodies) = setup_cubes(
        Cube::new(1.0, 2.0, 1.0),
        State::new_stationary(),
        Cube::new(2.0, 1.0, 1.0),
        State::new_with_position(1.49, 1.49, 0.99),
    );

    let intersection = narrowphase.find_intersection(&bodies[0], &bodies[1]);

    assert!(intersection.is_some());
    // TODO officially support vertex – vertex contacts
    // let c = intersection.unwrap();
    // assert_eq!(c.normal, Vector::new(1.0, 0.0, 0.0));
    // assert_eq!(c.point, Vector::new(0.5, 0.0, 0.0));
}

#[test]
fn almost_colliding_edge_to_edge() {
    let (narrowphase, bodies) = setup_cubes(
        Cube::new(1.0, 2.0, 3.0),
        State::new_stationary(),
        Cube::new(1.0, 2.0, 3.0),
        State::new_with_position(1.01, 1.51, 0.00),
    );

    let intersection = narrowphase.find_intersection(&bodies[0], &bodies[1]);

    assert!(intersection.is_none());
}

#[test]
fn colliding_edge_to_edge() {
    let (narrowphase, bodies) = setup_cubes(
        Cube::new(1.0, 2.0, 3.0),
        State::new_stationary(),
        Cube::new(1.0, 2.0, 3.0),
        State::new_with_position(0.99, 1.49, 0.00),
    );

    let intersection = narrowphase.find_intersection(&bodies[0], &bodies[1]);

    assert!(intersection.is_some());
    // TODO officially support edge – edge contacts
    // let c = intersection.unwrap();
    // assert!(c.point.distance_to(Vector::new(0.5, 0.0, 0.0)) < 0.001);
    // assert!(c.normal.distance_to(Vector::new(1.0, 0.0, 0.0)) < 0.001);
}

#[test]
fn almost_colliding_face_to_face() {
    let (narrowphase, bodies) = setup_cubes(
        Cube::new(1.0, 1.0, 1.0),
        State::new_stationary(),
        Cube::new(1.0, 1.0, 1.0),
        State::new_with_position(1.01, 0.5, 0.5),
    );

    let intersection = narrowphase.find_intersection(&bodies[0], &bodies[1]);

    assert!(intersection.is_none());
}

#[test]
fn colliding_face_to_face() {
    let (narrowphase, bodies) = setup_cubes(
        Cube::new(1.0, 1.0, 1.0),
        State::new_stationary(),
        Cube::new(1.0, 1.0, 1.0),
        State::new_with_position(0.99, 0.5, 0.5),
    );

    let option = narrowphase.find_intersection(&bodies[0], &bodies[1]);

    assert!(option.is_some());
    let intersection = option.unwrap();
    assert_eq!(intersection.normal(), Vector::new(1.0, 0.0, 0.0));
    // TODO compute intersection point
    // assert_eq!(c.point, Vector::new(0.995, 0.750, 0.750));
}

#[test]
fn almost_colliding_edge_to_face() {
    let (narrowphase, bodies) = setup_cubes(
        Cube::new(1.0, 1.0, 1.0),
        State::new_stationary(),
        Cube::new(1.0, 1.0, 1.0),
        State::new_with_position(0.51 + 0.5*2.0f32.sqrt(), 0.00, 0.00)
            .with_axis_angle(Vector::new(0.0, 1.0, 0.0), PI/4.0),
    );

    let intersection = narrowphase.find_intersection(&bodies[0], &bodies[1]);

    assert!(intersection.is_none());
}

#[test]
fn colliding_edge_to_face() {
    let (narrowphase, bodies) = setup_cubes(
        Cube::new(1.0, 1.0, 1.0),
        State::new_stationary(),
        Cube::new(1.0, 1.0, 1.0),
        State::new_with_position(0.49 + 0.5*2.0f32.sqrt(), 0.00, 0.00)
            .with_axis_angle(Vector::new(0.0, 0.0, 1.0), PI/4.0),
    );

    let option = narrowphase.find_intersection(&bodies[0], &bodies[1]);

    assert!(option.is_some());

    let intersection = option.unwrap();
    assert_eq!(intersection.normal(), Vector::new(1.0, 0.0, 0.0));
    // TODO compute intersection point
    // assert_eq!(c.point, Vector::new(0.5, 0.0, 0.0));
}

#[test]
fn almost_colliding_vertex_to_face() {
    // SETUP
    let initial_axis = Vector::new(1.0, 1.0, 1.0).normalize();
    let final_axis = Vector::new(1.0, 0.0, 0.0);
    let rotation = initial_axis.cross(final_axis);
    let state_1 = State::new_with_position((1.01 + 3.0f32.sqrt())/2.0, 0.0, 0.0)
        .with_axis_angle(rotation, rotation.length().asin());
    let (narrowphase, bodies) = setup_cubes(
        Cube::new(1.0, 1.0, 1.0),
        State::new_stationary(),
        Cube::new(1.0, 1.0, 1.0),
        state_1,
    );

    // EXERCISE
    let intersection = narrowphase.find_intersection(&bodies[0], &bodies[1]);

    // VERIFY
    assert!(intersection.is_none());
}

#[test]
fn colliding_vertex_to_face() {
    // SETUP
    let initial_axis = Vector::new(1.0, 1.0, 1.0).normalize();
    let final_axis = Vector::new(1.0, 0.0, 0.0);
    let rotation = initial_axis.cross(final_axis);
    let state_1 = State::new_with_position((0.98 + 3.0f32.sqrt())/2.0, 0.1, 0.0)
        .with_axis_angle(rotation, rotation.length().asin());
    let (narrowphase, bodies) = setup_cubes(
        Cube::new(1.0, 1.0, 1.0),
        State::new_stationary(),
        Cube::new(1.0, 1.0, 1.0),
        state_1,
    );

    let option = narrowphase.find_intersection(&bodies[0], &bodies[1]);

    assert!(option.is_some());

    let intersection = option.unwrap();
    assert_eq!(intersection.normal(), Vector::new(1.0, 0.0, 0.0));
    assert_eq!(intersection.point(), Vector::new(0.495, 0.1, 0.0));
}
