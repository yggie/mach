use mach::core::State;
use mach::shapes::Cube;
use mach::materials::Rigid;
use mach::collisions::Collisions;

pub fn creating_a_rigid_body<C: Collisions<Identifier=usize>, F: FnOnce() -> C>(new_collisions: F) {
    let mut collisions = new_collisions();
    let shape = Cube::new(1.0, 1.0, 1.0);
    let material = Rigid::new(3.0);

    let uid = collisions.create_body(shape.clone(), material, State::new_stationary());

    assert!(collisions.find_body(uid).is_some());
}

pub fn finding_a_body_with_a_handle<C: Collisions<Identifier=usize>, F: FnOnce() -> C>(new_collisions: F) {
    let mut collisions = new_collisions();
    let shape = Cube::new(1.0, 1.0, 1.0);
    let material = Rigid::new(3.0);
    collisions.create_body(shape.clone(), material, State::new_stationary());
    let uid = collisions.create_body(shape.clone(), material, State::new_stationary());
    collisions.create_body(shape.clone(), material, State::new_stationary());

    let body = collisions.find_body(uid);

    assert_eq!(body.unwrap().id(), uid);
}

pub fn mutably_finding_a_body_with_a_handle<C: Collisions<Identifier=usize>, F: FnOnce() -> C>(new_collisions: F) {
    let mut collisions = new_collisions();
    let shape = Cube::new(1.0, 1.0, 1.0);
    let material = Rigid::new(3.0);
    let uid = collisions.create_body(shape.clone(), material, State::new_stationary());
    collisions.create_body(shape.clone(), material, State::new_stationary());
    collisions.create_body(shape.clone(), material, State::new_stationary());

    let body = collisions.find_body_mut(uid);

    assert_eq!(body.unwrap().id(), uid);
}

pub fn iterating_over_bodies<C: Collisions<Identifier=usize>, F: FnOnce() -> C>(new_collisions: F) {
    let mut collisions = new_collisions();
    let shape = Cube::new(1.0, 1.0, 1.0);
    let material = Rigid::new(3.0);
    let mut uids = vec!(
        collisions.create_body(shape.clone(), material, State::new_stationary()),
        collisions.create_body(shape.clone(), material, State::new_stationary()),
        collisions.create_body(shape.clone(), material, State::new_stationary()),
    );

    let mut iterated_uids: Vec<usize> = collisions.bodies_iter()
        .map(|body| body.id())
        .collect();

    uids.sort_by(|a, b| a.cmp(&b));
    iterated_uids.sort_by(|a, b| a.cmp(&b));
    for (uid, expected_uid) in iterated_uids.iter().zip(uids.iter()) {
        assert_eq!(uid, expected_uid);
    }
}

pub fn mutably_iterating_over_bodies<C: Collisions<Identifier=usize>, F: FnOnce() -> C>(new_collisions: F) {
    let mut collisions = new_collisions();
    let shape = Cube::new(1.0, 1.0, 1.0);
    let material = Rigid::new(3.0);
    let mut uids = vec!(
        collisions.create_body(shape.clone(), material, State::new_stationary()),
        collisions.create_body(shape.clone(), material, State::new_stationary()),
        collisions.create_body(shape.clone(), material, State::new_stationary()),
    );

    let mut iterated_uids: Vec<usize> = collisions.bodies_iter_mut()
        .map(|body| body.id())
        .collect();

    uids.sort_by(|a, b| a.cmp(&b));
    iterated_uids.sort_by(|a, b| a.cmp(&b));
    for (uid, expected_uid) in iterated_uids.iter().zip(uids.iter()) {
        assert_eq!(uid, expected_uid);
    }
}

macro_rules! assert_collisions_behaviour(
    ($new_collisions:expr) => (
        #[test]
        fn creating_a_rigid_body() {
            behaviours::collisions_behaviour::creating_a_rigid_body($new_collisions);
        }

        #[test]
        fn finding_a_body_with_a_handle() {
            behaviours::collisions_behaviour::finding_a_body_with_a_handle($new_collisions);
        }

        #[test]
        fn mutably_finding_a_body_with_a_handle() {
            behaviours::collisions_behaviour::mutably_finding_a_body_with_a_handle($new_collisions);
        }

        #[test]
        fn iterating_over_bodies() {
            behaviours::collisions_behaviour::iterating_over_bodies($new_collisions);
        }

        #[test]
        fn mutably_iterating_over_bodies() {
            behaviours::collisions_behaviour::mutably_iterating_over_bodies($new_collisions);
        }
    );
);
