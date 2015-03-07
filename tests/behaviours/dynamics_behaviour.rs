use core::State;
use math::Vector;
use shapes::Cube;
use dynamics::Dynamics;
use materials::Rigid;
use collisions::{ Collisions, SimpleCollisions };

pub fn gravity_test<D: Dynamics, F: FnOnce() -> D>(new_dynamics: F) {
    // SETUP
    let mut dynamics = new_dynamics();
    let mut space = &mut SimpleCollisions::new();
    space.create_body(
        Cube::new(1.0, 1.0, 1.0),
        Rigid::new(1.0),
        State::new_stationary().with_velocity(1.0, -1.0, 0.5),
    );

    // EXERCISE
    dynamics.set_gravity(Vector::new(2.5, -2.5, 3.3));

    // VERIFY
    assert_eq!(dynamics.gravity(), Vector::new(2.5, -2.5, 3.3));
}

pub mod update {
    use core::State;
    use math::Vector;
    use utils::{ kinetic_energy_for };
    use shapes::Cube;
    use dynamics::Dynamics;
    use materials::Rigid;
    use collisions::{ Collisions, SimpleCollisions };
    use utils::log::{ CollisionsLogger, DynamicsLogger };

    fn assert_approximately_equal(a: Vector, b: Vector) {
        // uses a larger tolerance to accommodate different algorithms
        assert!(a.distance_to(b) < 0.01, format!("Expected {} to be approximately equal to {}", a, b));
    }

    pub fn constant_velocity_test<D: Dynamics, F: FnOnce() -> D>(new_dynamics: F) {
        // SETUP
        let mut dynamics = new_dynamics();
        let mut space = &mut SimpleCollisions::new();
        let uid = space.create_body(
            Cube::new(1.0, 1.0, 1.0),
            Rigid::new(1.0),
            State::new_stationary().with_velocity(1.0, -1.0, 0.5),
        );

        // EXERCISE
        println!("[Update: START] step: {}", 0.3);
        dynamics.update(space, 0.3);
        for body in space.bodies_iter() {
            println!(
                "[Body: STATE] {}: pos: {}, rot: {}",
                body.handle(),
                body.position(),
                body.rotation_quaternion()
            );
        }
        println!("[Update: END]");

        // VERIFY
        let body = space.find_body(uid).unwrap();
        assert_eq!(body.position(), Vector::new(0.30, -0.30, 0.15));
        assert_eq!(body.velocity(), Vector::new(1.0, -1.0, 0.5));
    }

    pub fn with_gravity_test<D: Dynamics, F: FnOnce() -> D>(new_dynamics: F) {
        // SETUP
        let mut dynamics = new_dynamics();
        let mut space = &mut SimpleCollisions::new();
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

    pub fn with_contact_test<D: Dynamics, F: FnOnce() -> D>(new_dynamics: F) {
        // SETUP
        let mut dynamics = DynamicsLogger::new(new_dynamics());
        let mut space = &mut CollisionsLogger::new(SimpleCollisions::new());
        let uid_0 = space.create_body(
            Cube::new(1.0, 1.0, 1.0),
            Rigid::new(1.0),
            State::new_stationary(),
        );
        let initial_axis = Vector::new(1.0, 1.0, 1.0).normalize();
        let final_axis = Vector::new(1.0, 0.0, 0.0);
        let rotation = initial_axis.cross(final_axis);
        let state_1 = State::new_with_position((0.98 + 3.0f32.sqrt())/2.0, 0.0, 0.0)
            .with_rotation(rotation, rotation.length().asin())
            .with_velocity(-1.0, 0.0, 0.0);
        let uid_1 = space.create_body(
            Cube::new(1.0, 1.0, 1.0),
            Rigid::new(1.0),
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

    pub fn with_rotating_contact_test<D: Dynamics, F: FnOnce() -> D>(new_dynamics: F) {
        // SETUP
        let mut dynamics = DynamicsLogger::new(new_dynamics());
        let mut space = &mut CollisionsLogger::new(SimpleCollisions::new());
        let uid_0 = space.create_body(
            Cube::new(1.0, 1.0, 1.0),
            Rigid::new(1.0),
            State::new_stationary(),
        );
        let initial_axis = Vector::new(1.0, 1.0, 1.0).normalize();
        let final_axis = Vector::new(1.0, 0.0, 0.0);
        let rotation = initial_axis.cross(final_axis);
        let state_1 = State::new_with_position((0.98 + 3.0f32.sqrt())/2.0, 0.0, 0.2)
            .with_rotation(rotation, rotation.length().asin())
            .with_velocity(-1.0, 0.0, 0.0);
        let uid_1 = space.create_body(
            Cube::new(1.0, 1.0, 1.0),
            Rigid::new(1.0),
            state_1,
        );

        let ke_before_0;
        let ke_before_1;
        let total_ke_before;
        {
            let body_0 = space.find_body(uid_0).unwrap();
            let body_1 = space.find_body(uid_1).unwrap();
            ke_before_0 = kinetic_energy_for(&body_0);
            ke_before_1 = kinetic_energy_for(&body_1);
            total_ke_before = ke_before_0 + ke_before_1;
        }

        // EXERCISE
        dynamics.update(space, 0.2);

        // VERIFY
        let body_0 = space.find_body(uid_0).unwrap();
        let body_1 = space.find_body(uid_1).unwrap();
        let ke_after_0 = kinetic_energy_for(&body_0);
        let ke_after_1 = kinetic_energy_for(&body_1);
        let total_ke_after = ke_after_0 + ke_after_1;

        assert!(total_ke_after < total_ke_before);
        println!("AFTER BODY 0: VEL: {} => {}", body_0.velocity(), body_0.velocity().length());
        println!("AFTER BODY 0: ANG VEL: {} => {}", body_0.angular_velocity(), body_0.angular_velocity().length());
        println!("AFTER BODY 1: VEL: {} => {}", body_1.velocity(), body_1.velocity().length());
        println!("AFTER BODY 1: ANG VEL: {} => {}", body_1.angular_velocity(), body_1.angular_velocity().length());
        assert_eq!(body_0.velocity(), Vector::new(-1.0, 0.0, 0.0));
        assert_eq!(body_1.velocity(), Vector::new( 0.0, 0.0, 0.0));
    }
}

macro_rules! assert_dynamics_behaviour(
    ($new_dynamics:expr) => (
        #[test]
        fn gravity_test() {
            behaviours::gravity_test($new_dynamics);
        }

        #[test]
        fn update_constant_velocity_test() {
            behaviours::update::constant_velocity_test($new_dynamics);
        }

        #[test]
        fn update_with_gravity_test() {
            behaviours::update::with_gravity_test($new_dynamics);
        }

        #[test]
        fn update_with_contact_test() {
            behaviours::update::with_contact_test($new_dynamics);
        }

        #[test]
        fn update_with_rotating_contact_test() {
            behaviours::update::with_rotating_contact_test($new_dynamics);
        }
    );
);
