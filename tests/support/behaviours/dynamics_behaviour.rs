macro_rules! assert_dynamics_behaviour(
    { $( $lines:item )+ } => (

        $( $lines )+

        mod dynamics_behaviour {
            use super::test_subject;

            use mach::{CustomWorld, Scalar, PI, World};
            use mach::maths::Vect;
            use mach::dynamics::Dynamics;
            use mach::entities::BodyParams;
            use mach::detection::{Space, MachSpace};

            fn new_world<D: Dynamics>(dynamics: D) -> CustomWorld<MachSpace, D> {
                return CustomWorld::new(MachSpace::new(), dynamics);
            }

            fn default_params() -> BodyParams {
                BodyParams::default().with_density(1.0)
                    .with_restitution_coefficient(1.0)
                    .with_friction_coefficient(0.0)
            }

            #[test]
            pub fn it_can_define_gravity() {
                let mut world = new_world(test_subject());

                world.set_gravity(Vect::new(2.5, -2.5, 3.3));

                assert_approx_eq!(world.gravity(), Vect::new(2.5, -2.5, 3.3));
            }

            #[test]
            pub fn it_can_simulate_objects_moving_at_constant_velocity() {
                let mut world = new_world(test_subject());
                let params = default_params();

                let id = world.create_rigid_body(
                    &params.as_cube(1.0)
                        .with_velocity(1.0, -1.0, 0.5)
                );

                world.update(0.3);

                let body = world.find_rigid_body(id).unwrap();
                assert_approx_eq!(body.translation(), Vect::new(0.30, -0.30, 0.15));
                assert_approx_eq!(body.velocity(), Vect::new(1.0, -1.0, 0.5));
            }

            #[test]
            pub fn it_can_simulate_objects_moving_at_constant_velocity_with_gravity() {
                let mut world = new_world(test_subject());
                let id = world.create_rigid_body(
                    &default_params().as_cube(1.0)
                        .with_velocity(1.0, -1.0, 0.5)
                );
                world.set_gravity(Vect::new(3.0, -2.0, 4.0));

                world.update(0.2);

                let body = world.find_rigid_body(id).unwrap();
                assert_approx_eq!(body.translation(), Vect::new(0.32, -0.28, 0.26));
                assert_approx_eq!(body.velocity(), Vect::new(1.6, -1.4, 1.3));
            }

            #[test]
            pub fn it_can_simulate_objects_colliding_without_rotation() {
                let mut world = new_world(test_subject());
                let params = default_params().as_cube(1.0);
                let id_0 = world.create_rigid_body(&params);
                let initial_axis = Vect::new(1.0, 1.0, 1.0).normalize();
                let final_axis = Vect::new(1.0, 0.0, 0.0);
                let rotation = initial_axis.cross(final_axis);

                let id_1 = world.create_rigid_body(
                    &params
                        .with_translation((0.98 + (3.0 as Scalar).sqrt())/2.0, 0.0, 0.0)
                        .with_axis_angle(rotation, rotation.length().asin())
                        .with_velocity(-1.0, 0.0, 0.0)
                );

                world.update(0.2);

                let body_0 = world.find_rigid_body(id_0).unwrap();
                let body_1 = world.find_rigid_body(id_1).unwrap();
                assert_approx_eq!(body_0.velocity(), Vect::new(-1.0, 0.0, 0.0));
                assert_approx_eq!(body_0.angular_velocity(), Vect::new(0.0, 0.0, 0.0));
                assert_approx_eq!(body_1.velocity(), Vect::new( 0.0, 0.0, 0.0));
                assert_approx_eq!(body_1.angular_velocity(), Vect::new(0.0, 0.0, 0.0));
            }

            #[test]
            pub fn it_can_simulate_objects_colliding_with_rotation() {
                let mut world = new_world(test_subject());
                let id_0 = world.create_rigid_body(
                    &default_params().as_cuboid(1.0, 10.0, 1.0)
                        .with_axis_angle(Vect::new(0.0, 1.0, 0.0), PI / 4.0)
                        .with_angular_velocity(-1.0, 0.0, 0.0)
                );
                world.create_static_body(
                    &default_params().as_cube(2.0)
                        .with_translation(0.0, 5.0, -1.05 - (0.5 as Scalar).sqrt())
                );

                world.update(0.05);

                // TODO quite a rough test, can be improved
                let rigid_body = world.find_rigid_body(id_0).unwrap();
                assert!(rigid_body.angular_velocity().dot(Vect::new(1.0, 0.0, 0.0)) > 0.0);
                assert!(rigid_body.velocity().dot(Vect::new(0.0, 0.0, 1.0)) > 0.0);
            }
        }
    );
);
