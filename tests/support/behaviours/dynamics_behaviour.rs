macro_rules! assert_dynamics_behaviour(
    { $( $lines:item )+ } => (

        $( $lines )+

        mod dynamics_behaviour {
            use super::test_subject;

            use support::MonitoredWorld;

            use mach::{EntityDesc, Scalar, PI, World};
            use mach::maths::Vector;
            use mach::dynamics::Dynamics;
            use mach::collisions::{CollisionSpace, SimpleCollisionSpace};

            fn new_world<D: Dynamics>(dynamics: D) -> MonitoredWorld<SimpleCollisionSpace, D> {
                return MonitoredWorld::new(SimpleCollisionSpace::new(), dynamics);
            }

            fn default_entity_desc() -> EntityDesc {
                EntityDesc::default().with_density(1.0)
                    .with_restitution_coefficient(1.0)
                    .with_friction_coefficient(0.0)
            }

            #[test]
            pub fn it_can_define_gravity() {
                let mut world = new_world(test_subject());

                world.set_gravity(Vector::new(2.5, -2.5, 3.3));

                assert_approx_eq!(world.gravity(), Vector::new(2.5, -2.5, 3.3));
            }

            #[test]
            pub fn it_can_simulate_objects_moving_at_constant_velocity() {
                let mut world = new_world(test_subject());
                let entity_desc = default_entity_desc();

                let id = world.create_body(
                    &entity_desc.as_cube(1.0)
                        .with_vel(1.0, -1.0, 0.5)
                );

                world.update(0.3);

                let body = world.find_body(id).unwrap();
                assert_approx_eq!(body.pos(), Vector::new(0.30, -0.30, 0.15));
                assert_approx_eq!(body.vel(), Vector::new(1.0, -1.0, 0.5));
            }

            #[test]
            pub fn it_can_simulate_objects_moving_at_constant_velocity_with_gravity() {
                let mut world = new_world(test_subject());
                let id = world.create_body(
                    &default_entity_desc().as_cube(1.0)
                        .with_vel(1.0, -1.0, 0.5)
                );
                world.set_gravity(Vector::new(3.0, -2.0, 4.0));

                world.update(0.2);

                let body = world.find_body(id).unwrap();
                assert_approx_eq!(body.pos(), Vector::new(0.32, -0.28, 0.26));
                assert_approx_eq!(body.vel(), Vector::new(1.6, -1.4, 1.3));
            }

            #[test]
            pub fn it_can_simulate_objects_colliding_without_rotation() {
                let mut world = new_world(test_subject());
                let entity_desc = default_entity_desc().as_cube(1.0);
                let id_0 = world.create_body(&entity_desc);
                let initial_axis = Vector::new(1.0, 1.0, 1.0).normalize();
                let final_axis = Vector::new(1.0, 0.0, 0.0);
                let rotation = initial_axis.cross(final_axis);

                let id_1 = world.create_body(
                    &entity_desc
                        .with_pos((0.98 + (3.0 as Scalar).sqrt())/2.0, 0.0, 0.0)
                        .with_axis_angle(rotation, rotation.length().asin())
                        .with_vel(-1.0, 0.0, 0.0)
                );

                world.update(0.2);

                let body_0 = world.find_body(id_0).unwrap();
                let body_1 = world.find_body(id_1).unwrap();
                assert_approx_eq!(body_0.vel(), Vector::new(-1.0, 0.0, 0.0));
                assert_approx_eq!(body_0.ang_vel(), Vector::new(0.0, 0.0, 0.0));
                assert_approx_eq!(body_1.vel(), Vector::new( 0.0, 0.0, 0.0));
                assert_approx_eq!(body_1.ang_vel(), Vector::new(0.0, 0.0, 0.0));
            }

            #[test]
            pub fn it_can_simulate_objects_colliding_with_rotation() {
                println!("[RENDERABLE]");
                let mut world = new_world(test_subject());
                let id_0 = world.create_body(
                    &default_entity_desc().as_cuboid(1.0, 10.0, 1.0)
                        .with_axis_angle(Vector::new(0.0, 1.0, 0.0), PI / 4.0)
                        .with_ang_vel(-1.0, 0.0, 0.0)
                );
                world.create_static_body(
                    &default_entity_desc().as_cube(2.0)
                        .with_pos(0.0, 5.0, -1.05 - (0.5 as Scalar).sqrt())
                );

                world.update(0.05);

                // TODO quite a rough test, can be improved
                let rigid_body = world.find_body(id_0).unwrap();
                assert!(rigid_body.ang_vel().dot(Vector::new(1.0, 0.0, 0.0)) > 0.0);
                assert!(rigid_body.vel().dot(Vector::new(0.0, 0.0, 1.0)) > 0.0);
            }
        }
    );
);
