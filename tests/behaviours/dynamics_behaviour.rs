use core::State;
use math::Vector;
use shapes::Sphere;
use materials::Rigid;
use dynamics::Dynamics;
use space::{ Space, SimpleSpace };

pub fn constant_velocity_test<D: Dynamics>(new_dynamics: || -> D) {
    // SETUP
    const NUM_POINTS: uint = 5u;
    const TIME_STEP: f32 = 0.3;
    let mut dynamics = new_dynamics();
    let mut space = &mut SimpleSpace::new();
    let mut points = [Vector::new_zero(), ..NUM_POINTS];
    let uid = space.create_body(
        Sphere::new(1.0),
        Rigid::new(1.0),
        State::new_stationary().with_velocity(1.0, -1.0, 0.5),
    );

    // EXERCISE
    for i in range(0u, NUM_POINTS) {
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

// pub fn constant_force_test<D: Dynamics>(new_dynamics: || -> D) {
//     // SETUP
//     let mut dynamics = new_dynamics();
//     let mut space = &mut SimpleSpace::new();
//     let tolerance = 0.10; // allow tolerance for different integration techniques
//     space.create_body(
//         Sphere::new(1.0),
//         Rigid::new(1.0),
//         State::new_stationary().with_velocity(1.0, -1.0, 0.5),
//     );
//     b.apply_impulse(Vector::new(2.5, -2.5, 3.3));
//
//     // EXERCISE
//     integrator(&mut b, 0.2);
//
//     if b.distance_to(Vector::new(0.30, -0.30, 0.15)) > tolerance {
//         panic!("Expected {} to be less than {}", diff.length(), tolerance);
//     }
//
//     if b.distance_to(Vector::new(1.50, -1.50, 1.16)) > tolerance {
//         panic!("Velocity change failed tolerance test: {} should be less than {}",
//              diff_vel.length(), tolerance);
//     }
// }

macro_rules! assert_dynamics_behaviour(
    ($new_dynamics:expr) => (
        #[test]
        fn constant_velocity_test() {
            behaviours::constant_velocity_test($new_dynamics);
        }
    );
);
