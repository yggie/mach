use mach::core::{ State, UID };
use mach::shapes::Cuboid;
use mach::entities::Material;
use mach::collisions::CollisionSpace;

pub fn creating_a_rigid_body<C: CollisionSpace, F: FnOnce() -> C>(new_collisions: F) {
    let mut collisions = new_collisions();
    let shape = Cuboid::new_cube(1.0);
    let material = &Material::new_with_density(3.0);

    let uid = collisions.create_body(shape.clone(), material, State::new_stationary());

    assert!(collisions.find_body(uid).is_some());
}

pub fn finding_a_body_with_a_handle<C: CollisionSpace, F: FnOnce() -> C>(new_collisions: F) {
    let mut collisions = new_collisions();
    let shape = Cuboid::new_cube(1.0);
    let material = &Material::new_with_mass(3.0);
    collisions.create_body(shape.clone(), material, State::new_stationary());
    let uid = collisions.create_body(shape.clone(), material, State::new_stationary());
    collisions.create_body(shape.clone(), material, State::new_stationary());

    let body = collisions.find_body(uid);

    assert_eq!(body.unwrap().id(), uid);
}

pub fn mutably_finding_a_body_with_a_handle<C: CollisionSpace, F: FnOnce() -> C>(new_collisions: F) {
    let mut collisions = new_collisions();
    let shape = Cuboid::new_cube(1.0);
    let material = &Material::new_with_density(3.0);
    let uid = collisions.create_body(shape.clone(), material, State::new_stationary());
    collisions.create_body(shape.clone(), material, State::new_stationary());
    collisions.create_body(shape.clone(), material, State::new_stationary());

    let body = collisions.find_body_mut(uid);

    assert_eq!(body.unwrap().id(), uid);
}

pub fn iterating_over_bodies<C: CollisionSpace, F: FnOnce() -> C>(new_collisions: F) {
    let mut collisions = new_collisions();
    let shape = Cuboid::new_cube(1.0);
    let material = &Material::new_with_mass(3.0);
    let mut uids = vec!(
        collisions.create_body(shape.clone(), material, State::new_stationary()),
        collisions.create_body(shape.clone(), material, State::new_stationary()),
        collisions.create_body(shape.clone(), material, State::new_stationary()),
    );

    let mut iterated_uids: Vec<UID> = collisions.bodies_iter()
        .map(|body| body.id())
        .collect();

    uids.sort_by(|a, b| a.cmp(&b));
    iterated_uids.sort_by(|a, b| a.cmp(&b));
    for (uid, expected_uid) in iterated_uids.iter().zip(uids.iter()) {
        assert_eq!(uid, expected_uid);
    }
}

pub fn mutably_iterating_over_bodies<C: CollisionSpace, F: FnOnce() -> C>(new_collisions: F) {
    let mut collisions = new_collisions();
    let shape = Cuboid::new_cube(1.0);
    let material = &Material::new_with_density(3.0);
    let mut uids = vec!(
        collisions.create_body(shape.clone(), material, State::new_stationary()),
        collisions.create_body(shape.clone(), material, State::new_stationary()),
        collisions.create_body(shape.clone(), material, State::new_stationary()),
    );

    let mut iterated_uids: Vec<UID> = collisions.bodies_iter_mut()
        .map(|body| body.id())
        .collect();

    uids.sort_by(|a, b| a.cmp(&b));
    iterated_uids.sort_by(|a, b| a.cmp(&b));
    for (uid, expected_uid) in iterated_uids.iter().zip(uids.iter()) {
        assert_eq!(uid, expected_uid);
    }
}

macro_rules! assert_collision_space_behaviour(
    ($new_collisions:expr) => (
        #[test]
        fn creating_a_rigid_body() {
            behaviours::collision_space_behaviour::creating_a_rigid_body($new_collisions);
        }

        #[test]
        fn finding_a_body_with_a_handle() {
            behaviours::collision_space_behaviour::finding_a_body_with_a_handle($new_collisions);
        }

        #[test]
        fn mutably_finding_a_body_with_a_handle() {
            behaviours::collision_space_behaviour::mutably_finding_a_body_with_a_handle($new_collisions);
        }

        #[test]
        fn iterating_over_bodies() {
            behaviours::collision_space_behaviour::iterating_over_bodies($new_collisions);
        }

        #[test]
        fn mutably_iterating_over_bodies() {
            behaviours::collision_space_behaviour::mutably_iterating_over_bodies($new_collisions);
        }
    );
);
