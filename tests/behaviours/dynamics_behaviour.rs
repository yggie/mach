use support::{ CollisionSpaceMonitor, DynamicsMonitor };

use mach::core::State;
use mach::maths::Vector;
use mach::shapes::Cube;
use mach::dynamics::Dynamics;
use mach::materials::Rigid;
use mach::collisions::{ CollisionSpace, SimpleCollisionSpace };


fn assert_approximately_equal(a: Vector, b: Vector) {
    // uses a larger tolerance to accommodate different algorithms
    assert!(a.distance_to(b) < 0.01, format!("Expected {} to be approximately equal to {}", a, b));
}

pub fn defining_gravity<D: Dynamics, F: FnOnce() -> D>(new_dynamics: F) {
    // SETUP
    let mut dynamics = new_dynamics();

    // EXERCISE
    dynamics.set_gravity(Vector::new(2.5, -2.5, 3.3));

    // VERIFY
    assert_eq!(dynamics.gravity(), Vector::new(2.5, -2.5, 3.3));
}

pub fn moving_at_constant_velocity<D: Dynamics, F: FnOnce() -> D>(new_dynamics: F) {
    // SETUP
    let mut dynamics = DynamicsMonitor::new(new_dynamics());
    let mut space = &mut CollisionSpaceMonitor::new(SimpleCollisionSpace::new());
    let uid = space.create_body(
        Cube::new(1.0, 1.0, 1.0),
        Rigid::new(1.0),
        State::new_stationary().with_velocity(1.0, -1.0, 0.5),
    );

    // EXERCISE
    dynamics.update(space, 0.3);

    // VERIFY
    let body = space.find_body(uid).unwrap();
    assert_eq!(body.position(), Vector::new(0.30, -0.30, 0.15));
    assert_eq!(body.velocity(), Vector::new(1.0, -1.0, 0.5));
}

pub fn moving_at_constant_velocity_with_gravity<D: Dynamics, F: FnOnce() -> D>(new_dynamics: F) {
    // SETUP
    let mut dynamics = DynamicsMonitor::new(new_dynamics());
    let mut space = &mut CollisionSpaceMonitor::new(SimpleCollisionSpace::new());
    let uid = space.create_body(
        Cube::new(1.0, 1.0, 1.0),
        Rigid::new(1.0),
        State::new_stationary().with_velocity(1.0, -1.0, 0.5),
    );
    dynamics.set_gravity(Vector::new(3.0, -2.0, 4.0));

    // EXERCISE
    dynamics.update(space, 0.2);

    let body = space.find_body(uid).unwrap();
    assert_approximately_equal(body.position(), Vector::new(0.32, -0.28, 0.26));
    assert_approximately_equal(body.velocity(), Vector::new(1.6, -1.4, 1.3));
}

pub fn moving_after_a_collision_without_rotation<D: Dynamics, F: FnOnce() -> D>(new_dynamics: F) {
    // SETUP
    let mut dynamics = DynamicsMonitor::new(new_dynamics());
    let mut space = &mut CollisionSpaceMonitor::new(SimpleCollisionSpace::new());
    let uid_0 = space.create_body(
        Cube::new(1.0, 1.0, 1.0),
        Rigid::new(1.0).with_coefficient_of_restitution(1.0),
        State::new_stationary(),
    );
    let initial_axis = Vector::new(1.0, 1.0, 1.0).normalize();
    let final_axis = Vector::new(1.0, 0.0, 0.0);
    let rotation = initial_axis.cross(final_axis);
    let state_1 = State::new_with_position((0.98 + 3.0f32.sqrt())/2.0, 0.0, 0.0)
        .with_axis_angle(rotation, rotation.length().asin())
        .with_velocity(-1.0, 0.0, 0.0);
    let uid_1 = space.create_body(
        Cube::new(1.0, 1.0, 1.0),
        Rigid::new(1.0).with_coefficient_of_restitution(1.0),
        state_1,
    );

    // EXERCISE
    dynamics.update(space, 0.2);

    // VERIFY
    let body_0 = space.find_body(uid_0).unwrap();
    let body_1 = space.find_body(uid_1).unwrap();
    assert_eq!(body_0.velocity(), Vector::new(-1.0, 0.0, 0.0));
    assert_eq!(body_0.angular_velocity(), Vector::new(0.0, 0.0, 0.0));
    assert_eq!(body_1.velocity(), Vector::new( 0.0, 0.0, 0.0));
    assert_eq!(body_1.angular_velocity(), Vector::new(0.0, 0.0, 0.0));
}

macro_rules! assert_dynamics_behaviour(
    ($new_dynamics:expr) => (
        #[test]
        fn defining_gravity() {
            behaviours::dynamics_behaviour::defining_gravity($new_dynamics);
        }

        #[test]
        fn moving_at_constant_velocity() {
            behaviours::dynamics_behaviour::moving_at_constant_velocity($new_dynamics);
        }

        #[test]
        fn moving_at_constant_velocity_with_gravity() {
            behaviours::dynamics_behaviour::moving_at_constant_velocity_with_gravity($new_dynamics);
        }

        #[test]
        fn moving_after_a_collision_without_rotation() {
            behaviours::dynamics_behaviour::moving_after_a_collision_without_rotation($new_dynamics);
        }
    );
);
