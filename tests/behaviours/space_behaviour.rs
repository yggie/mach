use core::State;
use space::Space;
use shapes::Cube;
use materials::Rigid;

pub fn create_body_test<S: Space, F: FnOnce() -> S>(new_space: F) {
    let mut space = new_space();
    let shape = Cube::new(1.0, 1.0, 1.0);
    let material = Rigid::new(3.0);
    space.create_body(shape.clone(), material, State::new_stationary());
    space.create_body(shape.clone(), material, State::new_with_position(0.99, 0.98, 0.98));

    // TODO implement a real test, now pair_test holds most of the specs
    assert!(true);
}

macro_rules! assert_space_behaviour(
    ($new_space:expr) => (
        #[test]
        fn create_body_test() {
            super::super::behaviours::create_body_test($new_space);
        }
    );
);
