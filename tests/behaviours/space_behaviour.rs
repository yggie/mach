use math::Vector;
use space::Space;
use shapes::Sphere;
use materials::Rigid;
use core::{ State };

pub fn sphere_sphere_contact_test<S: Space, F: FnOnce() -> S>(new_space: F) {
    // SETUP
    let mut space = new_space();
    let shape = Sphere::new(2.5);
    let material = Rigid::new(3.0);
    let states = [
        State::new_with_rotation(-1.3, 0.1, 0.0, 0.8),
        State::new_with_rotation(2.1, 0.5, 0.5, 1.0)
            .with_position(4.0, 3.0, 0.0),
    ];
    space.create_body(shape, material, states[0]);
    space.create_body(shape, material, states[1]);

    // EXERCISE
    let contacts = space.find_contacts();

    // VERIFY
    assert_eq!(contacts.len(), 1us);
    assert_eq!(contacts[0].point, Vector::new(2.0, 1.5, 0.0));
}

pub fn sphere_sphere_no_contact_test<S: Space, F: FnOnce() -> S>(new_space: F) {
    // SETUP
    let mut space = new_space();
    let shape = Sphere::new(2.5);
    let material = Rigid::new(3.0);
    space.create_body(shape, material, State::new_with_position(-0.05, -0.05, 0.00));
    space.create_body(shape, material, State::new_with_position(5.0, 0.0, 0.0));

    // EXERCISE
    let contacts = space.find_contacts();

    // VERIFY
    assert_eq!(contacts.len(), 0);
}

macro_rules! assert_space_behaviour(
    ($new_space:expr) => (
        #[test]
        fn sphere_sphere_contact_test() {
            behaviours::sphere_sphere_contact_test($new_space);
        }

        #[test]
        fn sphere_sphere_no_contact_test() {
            behaviours::sphere_sphere_no_contact_test($new_space);
        }
    );
);
