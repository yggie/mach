macro_rules! assert_dynamics_behaviour(
    { $( $lines:item )+ } => (

        $( $lines )+

        mod dynamics_behaviour {
            use super::test_subject;

            use support::{ CollisionSpaceMonitor, DynamicsMonitor };

            use mach::{ Float, PI, World };
            use mach::maths::{ State, Transform, Vector };
            use mach::shapes::Cuboid;
            use mach::dynamics::Dynamics;
            use mach::entities::Material;
            use mach::collisions::{ CollisionSpace, SimpleCollisionSpace };

            fn new_world<D: Dynamics>(dynamics: D) -> World<CollisionSpaceMonitor<SimpleCollisionSpace>, DynamicsMonitor<D>> {
                let monitored_dynamics = DynamicsMonitor::new(dynamics);
                let collision_space = CollisionSpaceMonitor::new(SimpleCollisionSpace::new());

                return World::new(collision_space, monitored_dynamics);
            }

            fn default_material() -> Material {
                Material::new_with_density(1.0)
                    .with_coefficient_of_restitution(1.0)
                    .with_friction_coefficient(0.0)
            }

            fn assert_approximately_equal(a: Vector, b: Vector) {
                // uses a larger tolerance to accommodate different algorithms
                assert!(a.distance_to(b) < 0.01, format!("Expected {} to be approximately equal to {}", a, b));
            }

            #[test]
            pub fn it_can_define_gravity() {
                let mut world = new_world(test_subject());

                world.set_gravity(Vector::new(2.5, -2.5, 3.3));

                assert_eq!(world.gravity(), Vector::new(2.5, -2.5, 3.3));
            }

            #[test]
            pub fn it_can_simulate_objects_moving_at_constant_velocity() {
                let mut world = new_world(test_subject());
                let id = world.create_body(
                    Cuboid::new_cube(1.0),
                    &default_material(),
                    State::new_stationary().with_velocity(1.0, -1.0, 0.5),
                );

                world.update(0.3);

                let body = world.find_body(id).unwrap();
                assert_eq!(body.position(), Vector::new(0.30, -0.30, 0.15));
                assert_eq!(body.velocity(), Vector::new(1.0, -1.0, 0.5));
            }

            #[test]
            pub fn it_can_simulate_objects_moving_at_constant_velocity_with_gravity() {
                let mut world = new_world(test_subject());
                let id = world.create_body(
                    Cuboid::new_cube(1.0),
                    &default_material(),
                    State::new_stationary().with_velocity(1.0, -1.0, 0.5),
                );
                world.set_gravity(Vector::new(3.0, -2.0, 4.0));

                world.update(0.2);

                let body = world.find_body(id).unwrap();
                assert_approximately_equal(body.position(), Vector::new(0.32, -0.28, 0.26));
                assert_approximately_equal(body.velocity(), Vector::new(1.6, -1.4, 1.3));
            }

            #[test]
            pub fn it_can_simulate_objects_colliding_without_rotation() {
                let mut world = new_world(test_subject());
                let id_0 = world.create_body(
                    Cuboid::new_cube(1.0),
                    &default_material(),
                    State::new_stationary(),
                );
                let initial_axis = Vector::new(1.0, 1.0, 1.0).normalize();
                let final_axis = Vector::new(1.0, 0.0, 0.0);
                let rotation = initial_axis.cross(final_axis);
                let state_1 = State::new_with_position((0.98 + (3.0 as Float).sqrt())/2.0, 0.0, 0.0)
                    .with_axis_angle(rotation, rotation.length().asin())
                    .with_velocity(-1.0, 0.0, 0.0);
                let id_1 = world.create_body(
                    Cuboid::new_cube(1.0),
                    &default_material(),
                    state_1,
                );

                world.update(0.2);

                let body_0 = world.find_body(id_0).unwrap();
                let body_1 = world.find_body(id_1).unwrap();
                assert_eq!(body_0.velocity(), Vector::new(-1.0, 0.0, 0.0));
                assert_eq!(body_0.angular_velocity(), Vector::new(0.0, 0.0, 0.0));
                assert_eq!(body_1.velocity(), Vector::new( 0.0, 0.0, 0.0));
                assert_eq!(body_1.angular_velocity(), Vector::new(0.0, 0.0, 0.0));
            }

            #[test]
            pub fn it_can_simulate_objects_colliding_with_rotation() {
                println!("[RENDERABLE]");
                let mut world = new_world(test_subject());
                let id_0 = world.create_body(
                    Cuboid::new(1.0, 10.0, 1.0),
                    &default_material(),
                    State::new_with_axis_angle(Vector::new(0.0, 1.0, 0.0), PI / 4.0)
                        .with_angular_velocity(-1.0, 0.0, 0.0),
                );
                world.create_static_body(
                    Cuboid::new_cube(2.0),
                    &default_material(),
                    Transform::new_with_translation(0.0, 5.0, -1.05 - (0.5 as Float).sqrt()),
                );

                world.update(0.05);

                // TODO quite a rough test, can be improved
                let rigid_body = world.find_body(id_0).unwrap();
                assert!(rigid_body.angular_velocity().dot(Vector::new(1.0, 0.0, 0.0)) > 0.0);
                assert!(rigid_body.velocity().dot(Vector::new(0.0, 0.0, 1.0)) > 0.0);
            }
        }
    );
);
