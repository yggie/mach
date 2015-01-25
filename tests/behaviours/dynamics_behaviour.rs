use core::State;
use math::Vector;
use shapes::Sphere;
use materials::Rigid;
use dynamics::Dynamics;
use space::{ Space, SimpleSpace };

pub fn constant_velocity_test<D: Dynamics, F: FnOnce() -> D>(new_dynamics: F) {
    // SETUP
    const NUM_POINTS: usize = 5us;
    const TIME_STEP: f32 = 0.3;
    let mut dynamics = new_dynamics();
    let mut space = &mut SimpleSpace::new();
    let mut points = [Vector::new_zero(); NUM_POINTS];
    let uid = space.create_body(
        Sphere::new(1.0),
        Rigid::new(1.0),
        State::new_stationary().with_velocity(1.0, -1.0, 0.5),
    );

    // EXERCISE
    for i in range(0us, NUM_POINTS) {
        dynamics.update(space, TIME_STEP);
        points[i] = space.find_body(uid).unwrap().position();
    }

    // VERIFY
    assert_eq!(points[0], Vector::new(0.30, -0.30, 0.15));
    assert_eq!(points[1], Vector::new(0.60, -0.60, 0.30));
    assert_eq!(points[2], Vector::new(0.90, -0.90, 0.45));
    assert_eq!(points[3], Vector::new(1.20, -1.20, 0.60));
    assert_eq!(points[4], Vector::new(1.50, -1.50, 0.75));
}

pub fn gravity_test<D: Dynamics, F: FnOnce() -> D>(new_dynamics: F) {
    // SETUP
    let mut dynamics = new_dynamics();
    let mut space = &mut SimpleSpace::new();
    let tolerance = 0.10; // allow tolerance for different integration techniques
    let uid = space.create_body(
        Sphere::new(1.0),
        Rigid::new(1.0),
        State::new_stationary().with_velocity(1.0, -1.0, 0.5),
    );
    dynamics.set_gravity(Vector::new(2.5, -2.5, 3.3));

    // EXERCISE
    dynamics.update(space, 0.2);

    let body = space.find_body_mut(uid).unwrap();
    let diff = body.position().distance_to(Vector::new(0.30, -0.30, 0.15));
    if diff > tolerance {
        panic!("Expected {} to be less than {}", diff, tolerance);
    }

    let diff_vel = body.velocity().distance_to(Vector::new(1.50, -1.50, 1.16));
    if diff_vel > tolerance {
        panic!("Velocity change failed tolerance test: {} should be less than {}",
             diff_vel, tolerance);
    }
}

pub fn constant_force_test<D: Dynamics, F: FnOnce() -> D>(new_dynamics: F) {
    // SETUP
    let mut dynamics = new_dynamics();
    let mut space = &mut SimpleSpace::new();
    let tolerance = 0.10; // allow tolerance for different integration techniques
    let uid = space.create_body(
        Sphere::new(1.0),
        Rigid::new(1.0),
        State::new_stationary().with_velocity(1.0, -1.0, 0.5),
    );
    {
        let body = space.find_body_mut(uid).unwrap();
        let p = body.position();
        body.apply_impulse(Vector::new(2.5, -2.5, 3.3), p + Vector::new(1.0, 0.0, 0.0));
    }

    // EXERCISE
    dynamics.update(space, 0.2);

    let body = space.find_body_mut(uid).unwrap();
    let diff = body.position().distance_to(Vector::new(0.70, -0.70, 0.76));
    if diff > tolerance {
        panic!("Expected {} to be less than {}", diff, tolerance);
    }

    let diff_vel = body.velocity().distance_to(Vector::new(3.50, -3.50, 3.8));
    if diff_vel > tolerance {
        panic!("Velocity change failed tolerance test: {} should be less than {}",
             diff_vel, tolerance);
    }
}

macro_rules! assert_dynamics_behaviour(
    ($new_dynamics:expr) => (
        #[test]
        fn constant_velocity_test() {
            behaviours::constant_velocity_test($new_dynamics);
        }

        #[test]
        fn constant_force_test() {
            behaviours::constant_force_test($new_dynamics);
        }

        #[test]
        fn gravity_test() {
            behaviours::gravity_test($new_dynamics);
        }
    );
);
