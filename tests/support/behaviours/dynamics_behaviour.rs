macro_rules! assert_dynamics_behaviour(
    { $( $lines:item )+ } => (

        $( $lines )+

        mod dynamics_behaviour {
            use super::test_subject;

            use support::{ CollisionSpaceMonitor, DynamicsMonitor };

            use mach::core::Float;
            use mach::maths::{ State, Vector };
            use mach::shapes::Cuboid;
            use mach::dynamics::Dynamics;
            use mach::entities::Material;
            use mach::collisions::{ CollisionSpace, SimpleCollisionSpace };

            fn validate<D: Dynamics>(object: D) -> D {
                object
            }

            fn assert_approximately_equal(a: Vector, b: Vector) {
                // uses a larger tolerance to accommodate different algorithms
                assert!(a.distance_to(b) < 0.01, format!("Expected {} to be approximately equal to {}", a, b));
            }

            #[test]
            pub fn it_can_define_gravity() {
                let mut dynamics = validate(test_subject());

                dynamics.set_gravity(Vector::new(2.5, -2.5, 3.3));

                assert_eq!(dynamics.gravity(), Vector::new(2.5, -2.5, 3.3));
            }

            #[test]
            pub fn it_can_simulate_objects_moving_at_constant_velocity() {
                let mut dynamics = DynamicsMonitor::new(validate(test_subject()));
                let mut space = &mut CollisionSpaceMonitor::new(SimpleCollisionSpace::new());
                let uid = space.create_body(
                    Cuboid::new_cube(1.0),
                    &Material::new_with_density(1.0),
                    State::new_stationary().with_velocity(1.0, -1.0, 0.5),
                );

                dynamics.update(space, 0.3);

                let body = space.find_body(uid).unwrap();
                assert_eq!(body.position(), Vector::new(0.30, -0.30, 0.15));
                assert_eq!(body.velocity(), Vector::new(1.0, -1.0, 0.5));
            }

            #[test]
            pub fn it_can_simulate_objects_moving_at_constant_velocity_with_gravity() {
                let mut dynamics = DynamicsMonitor::new(validate(test_subject()));
                let mut space = &mut CollisionSpaceMonitor::new(SimpleCollisionSpace::new());
                let uid = space.create_body(
                    Cuboid::new_cube(1.0),
                    &Material::new_with_density(1.0),
                    State::new_stationary().with_velocity(1.0, -1.0, 0.5),
                );
                dynamics.set_gravity(Vector::new(3.0, -2.0, 4.0));

                dynamics.update(space, 0.2);

                let body = space.find_body(uid).unwrap();
                assert_approximately_equal(body.position(), Vector::new(0.32, -0.28, 0.26));
                assert_approximately_equal(body.velocity(), Vector::new(1.6, -1.4, 1.3));
            }

            #[test]
            pub fn it_can_simulate_objects_colliding_without_rotation() {
                let mut dynamics = DynamicsMonitor::new(validate(test_subject()));
                let mut space = &mut CollisionSpaceMonitor::new(SimpleCollisionSpace::new());
                let uid_0 = space.create_body(
                    Cuboid::new_cube(1.0),
                    &Material::new_with_density(1.0).with_coefficient_of_restitution(1.0),
                    State::new_stationary(),
                );
                let initial_axis = Vector::new(1.0, 1.0, 1.0).normalize();
                let final_axis = Vector::new(1.0, 0.0, 0.0);
                let rotation = initial_axis.cross(final_axis);
                let state_1 = State::new_with_position((0.98 + (3.0 as Float).sqrt())/2.0, 0.0, 0.0)
                    .with_axis_angle(rotation, rotation.length().asin())
                    .with_velocity(-1.0, 0.0, 0.0);
                let uid_1 = space.create_body(
                    Cuboid::new_cube(1.0),
                    &Material::new_with_density(1.0).with_coefficient_of_restitution(1.0),
                    state_1,
                );

                dynamics.update(space, 0.2);

                let body_0 = space.find_body(uid_0).unwrap();
                let body_1 = space.find_body(uid_1).unwrap();
                assert_eq!(body_0.velocity(), Vector::new(-1.0, 0.0, 0.0));
                assert_eq!(body_0.angular_velocity(), Vector::new(0.0, 0.0, 0.0));
                assert_eq!(body_1.velocity(), Vector::new( 0.0, 0.0, 0.0));
                assert_eq!(body_1.angular_velocity(), Vector::new(0.0, 0.0, 0.0));
            }
        }
    );
);
