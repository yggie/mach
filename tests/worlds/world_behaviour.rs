macro_rules! assert_world_behaviour {
    { $( $lines:item )+ } => (
        $( $lines )+

        mod world_behaviour {
            use super::test_subject;

            use {PI, Scalar, World};
            use maths::{CrossProduct, DotProduct, UnitQuat, Vec3D};
            use shapes::Cuboid;
            use collisions::CollisionBody;
            use dynamics::{DynamicBodyExtension, FixedBodyDef, RigidBodyDef, RigidBodyRef};

            #[test]
            fn it_can_simulate_constant_velocity() {
                let mut world = validate(test_subject());
                let handle = world.create_rigid_body(RigidBodyDef {
                    shape: Box::new(Cuboid::cube(1.0)),
                    velocity: Vec3D::new(1.0, -1.0, 0.5),
                    .. RigidBodyDef::default()
                }, ());

                world.update(0.3);

                let body = handle.borrow();
                let rigid_body = RigidBodyRef::try_from(&*body)
                    .expect("expected the body to be rigid but was not");

                assert_approx_eq!(rigid_body.translation(), Vec3D::new(0.30, -0.30, 0.15));
                assert_approx_eq!(rigid_body.velocity(), Vec3D::new(1.0, -1.0, 0.5));
            }

            #[test]
            fn it_can_simulate_collisions_without_rotation() {
                let mut world = validate(test_subject());
                let handle_0 = world.create_rigid_body(RigidBodyDef {
                    mass: 1.0,
                    shape: Box::new(Cuboid::cube(1.0)),
                    restitution_coefficient: 1.0,
                    .. RigidBodyDef::default()
                }, ());

                let initial_axis = Vec3D::new(1.0, 1.0, 1.0).normalize();
                let final_axis = Vec3D::new(1.0, 0.0, 0.0);
                let rotation = initial_axis.cross(final_axis);
                let handle_1 = world.create_rigid_body(RigidBodyDef {
                    mass: 1.0,
                    shape: Box::new(Cuboid::cube(1.0)),
                    rotation: UnitQuat::from_axis_angle(rotation.normalize(), rotation.length().asin()),
                    translation: Vec3D::new((0.98 + (3.0 as Scalar).sqrt())/2.0, 0.0, 0.0),
                    velocity: Vec3D::new(-1.0, 0.0, 0.0),
                    restitution_coefficient: 1.0,
                    .. RigidBodyDef::default()
                }, ());

                let contacts = world.update(0.2);

                // TODO check that the event is a contact event
                assert!(contacts.len() == 1);
                let body_0 = handle_0.borrow();
                let rigid_body_0 = RigidBodyRef::try_from(&*body_0)
                    .expect("expected the body to be rigid but was not");
                let body_1 = handle_1.borrow();
                let rigid_body_1 = RigidBodyRef::try_from(&*body_1)
                    .expect("expected the body to be rigid but was not");
                assert_approx_eq!(rigid_body_0.velocity(), Vec3D::new(-1.0, 0.0, 0.0));
                assert_approx_eq!(rigid_body_0.angular_velocity(), Vec3D::new(0.0, 0.0, 0.0));
                assert_approx_eq!(rigid_body_1.velocity(), Vec3D::new(0.0, 0.0, 0.0));
                assert_approx_eq!(rigid_body_1.angular_velocity(), Vec3D::new(0.0, 0.0, 0.0));
            }

            #[test]
            fn it_can_simulate_collisions_with_rotation() {
                let mut world = validate(test_subject());
                let handle_0 = world.create_rigid_body(RigidBodyDef {
                    mass: 1.0,
                    shape: Box::new(Cuboid::new(1.0, 10.0, 1.0)),
                    rotation: UnitQuat::from_axis_angle(Vec3D::new(0.0, 1.0, 0.0).normalize(), PI / 4.0),
                    angular_velocity: Vec3D::new(-1.0, 0.0, 0.0),
                    restitution_coefficient: 1.0,
                    .. RigidBodyDef::default()
                }, ());

                let _handle_1 = world.create_fixed_body(FixedBodyDef {
                    shape: Box::new(Cuboid::cube(2.0)),
                    translation: Vec3D::new(0.0, 5.0, -1.05 - (0.5 as Scalar).sqrt()),
                    restitution_coefficient: 1.0,
                    .. FixedBodyDef::default()
                }, ());

                let contacts = world.update(0.05);

                // TODO check that the event is a contact event
                let body_0 = handle_0.borrow();
                let rigid_body = RigidBodyRef::try_from(&*body_0)
                    .expect("expected the body to be rigid but was not");
                assert!(contacts.len() == 1);
                // TODO quite a rough test, can be improved
                assert!(rigid_body.angular_velocity().dot(Vec3D::new(1.0, 0.0, 0.0)) > 0.0);
                assert!(rigid_body.velocity().dot(Vec3D::new(0.0, 0.0, 1.0)) > 0.0);
            }

            fn validate<T, W>(input: W) -> W where T: CollisionBody<Extension=DynamicBodyExtension<()>>, W: World<T> {
                input
            }
        }
    );
}
