macro_rules! assert_world_behaviour {
    { $( $lines:item )+ } => (
        $( $lines )+

        mod world_behaviour {
            use super::test_subject;

            use {PI, Scalar, World};
            use maths::{DotProduct, Vec3D};
            use shapes::Cuboid;
            use entities::{EntityStore, RigidBody, StaticBody};

            fn validate<W: World>(input: W) -> W {
                input
            }

            #[test]
            fn it_can_simulate_constant_velocity() {
                let mut world = validate(test_subject());
                let rigid_body = RigidBody::default()
                    .with_shape(Cuboid::cube(1.0))
                    .with_velocity(1.0, -1.0, 0.5);
                let id = world.add_rigid_body(rigid_body);

                world.update(0.3);

                let rigid_body = world.find_rigid_body(id).unwrap();

                assert_approx_eq!(rigid_body.translation(), Vec3D::new(0.30, -0.30, 0.15));
                assert_approx_eq!(rigid_body.velocity(), Vec3D::new(1.0, -1.0, 0.5));
            }

            #[test]
            fn it_can_simulate_collisions_without_rotation() {
                let mut world = validate(test_subject());
                let prototype = RigidBody::default()
                    .with_shape(Cuboid::cube(1.0))
                    .with_mass(1.0);
                let rigid_body_0 = prototype.clone()
                    .with_mass(1.0)
                    .with_restitution_coefficient(1.0);
                let id_0 = world.add_rigid_body(rigid_body_0);

                let initial_axis = Vec3D::new(1.0, 1.0, 1.0).normalize();
                let final_axis = Vec3D::new(1.0, 0.0, 0.0);
                let rotation = initial_axis.cross(final_axis);
                let rigid_body_1 = prototype.clone()
                    .with_mass(1.0)
                    .with_restitution_coefficient(1.0)
                    .with_translation((0.98 + (3.0 as Scalar).sqrt())/2.0, 0.0, 0.0)
                    .with_axis_angle(rotation, rotation.length().asin())
                    .with_velocity(-1.0, 0.0, 0.0);
                let id_1 = world.add_rigid_body(rigid_body_1);

                world.update(0.2);

                let rigid_body_0 = world.find_rigid_body(id_0).unwrap();
                let rigid_body_1 = world.find_rigid_body(id_1).unwrap();
                assert_approx_eq!(rigid_body_0.velocity(), Vec3D::new(-1.0, 0.0, 0.0));
                assert_approx_eq!(rigid_body_0.angular_velocity(), Vec3D::new(0.0, 0.0, 0.0));
                assert_approx_eq!(rigid_body_1.velocity(), Vec3D::new(0.0, 0.0, 0.0));
                assert_approx_eq!(rigid_body_1.angular_velocity(), Vec3D::new(0.0, 0.0, 0.0));
            }

            #[test]
            fn it_can_simulate_collisions_with_rotation() {
                let mut world = validate(test_subject());
                let rigid_body = RigidBody::default()
                    .with_shape(Cuboid::new(1.0, 10.0, 1.0))
                    .with_mass(1.0)
                    .with_restitution_coefficient(1.0)
                    .with_axis_angle(Vec3D::new(0.0, 1.0, 0.0), PI / 4.0)
                    .with_angular_velocity(-1.0, 0.0, 0.0);
                let rigid_body_id = world.add_rigid_body(rigid_body);

                let static_body = StaticBody::default()
                    .with_shape(Cuboid::cube(2.0))
                    .with_restitution_coefficient(1.0)
                    .with_translation(0.0, 5.0, -1.05 - (0.5 as Scalar).sqrt());
                let _static_body_id = world.add_static_body(static_body);

                world.update(0.05);

                // TODO quite a rough test, can be improved
                let rigid_body = world.find_rigid_body(rigid_body_id).unwrap();
                assert!(rigid_body.angular_velocity().dot(Vec3D::new(1.0, 0.0, 0.0)) > 0.0);
                assert!(rigid_body.velocity().dot(Vec3D::new(0.0, 0.0, 1.0)) > 0.0);
            }
        }
    );
}
