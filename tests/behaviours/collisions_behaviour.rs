use core::{ State, UID };
use shapes::Cube;
use materials::Rigid;
use collisions::Collisions;

pub fn create_body_test<C: Collisions, F: FnOnce() -> C>(new_collisions: F) {
    let mut collisions = new_collisions();
    let shape = Cube::new(1.0, 1.0, 1.0);
    let material = Rigid::new(3.0);

    let uid = collisions.create_body(shape.clone(), material, State::new_stationary());

    assert!(collisions.find_body(uid).is_some());
}

pub fn find_body_test<C: Collisions, F: FnOnce() -> C>(new_collisions: F) {
    let mut collisions = new_collisions();
    let shape = Cube::new(1.0, 1.0, 1.0);
    let material = Rigid::new(3.0);
    collisions.create_body(shape.clone(), material, State::new_stationary());
    let uid = collisions.create_body(shape.clone(), material, State::new_stationary());
    collisions.create_body(shape.clone(), material, State::new_stationary());

    let body = collisions.find_body(uid);

    assert_eq!(body.unwrap().handle(), uid);
}

pub fn find_body_mut_test<C: Collisions, F: FnOnce() -> C>(new_collisions: F) {
    let mut collisions = new_collisions();
    let shape = Cube::new(1.0, 1.0, 1.0);
    let material = Rigid::new(3.0);
    let uid = collisions.create_body(shape.clone(), material, State::new_stationary());
    collisions.create_body(shape.clone(), material, State::new_stationary());
    collisions.create_body(shape.clone(), material, State::new_stationary());

    let body = collisions.find_body_mut(uid);

    assert_eq!(body.unwrap().handle(), uid);
}

pub fn bodies_iter_test<C: Collisions, F: FnOnce() -> C>(new_collisions: F) {
    let mut collisions = new_collisions();
    let shape = Cube::new(1.0, 1.0, 1.0);
    let material = Rigid::new(3.0);
    let mut uids = vec!(
        collisions.create_body(shape.clone(), material, State::new_stationary()),
        collisions.create_body(shape.clone(), material, State::new_stationary()),
        collisions.create_body(shape.clone(), material, State::new_stationary()),
    );

    let mut iterated_uids: Vec<UID> = collisions.bodies_iter()
        .map(|body| body.handle())
        .collect();

    uids.sort_by(|a, b| a.cmp(&b));
    iterated_uids.sort_by(|a, b| a.cmp(&b));
    for (uid, expected_uid) in iterated_uids.iter().zip(uids.iter()) {
        assert_eq!(uid, expected_uid);
    }
}

pub fn bodies_iter_mut_test<C: Collisions, F: FnOnce() -> C>(new_collisions: F) {
    let mut collisions = new_collisions();
    let shape = Cube::new(1.0, 1.0, 1.0);
    let material = Rigid::new(3.0);
    let mut uids = vec!(
        collisions.create_body(shape.clone(), material, State::new_stationary()),
        collisions.create_body(shape.clone(), material, State::new_stationary()),
        collisions.create_body(shape.clone(), material, State::new_stationary()),
    );

    let mut iterated_uids: Vec<UID> = collisions.bodies_iter_mut()
        .map(|body| body.handle())
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
        fn create_body_test() {
            super::super::behaviours::create_body_test($new_collisions);
        }

        #[test]
        fn find_body_test() {
            super::super::behaviours::find_body_test($new_collisions);
        }

        #[test]
        fn find_body_mut_test() {
            super::super::behaviours::find_body_mut_test($new_collisions);
        }

        #[test]
        fn bodies_iter_test() {
            super::super::behaviours::bodies_iter_test($new_collisions);
        }

        #[test]
        fn bodies_iter_mut_test() {
            super::super::behaviours::bodies_iter_mut_test($new_collisions);
        }
    );
);
