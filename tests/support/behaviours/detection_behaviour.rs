macro_rules! assert_detection_behaviour {
    ( $( $lines:item )+ ) => {
        $( $lines )+

        mod detection_behaviour {
            extern crate quickcheck;

            use super::test_subject;

            use {PI, Scalar};
            use maths::Vect;
            use shapes::Cuboid;
            use support::inputs;
            use entities::{Body, BodyHandle, RigidBody};
            use detection::Detection;

            fn validate<D: Detection>(input: D) -> D {
                input
            }

            #[test]
            fn it_does_not_return_false_positives() {
                fn property(offset: inputs::UnitVect, rot: inputs::UnitQuat) {
                    let mut detection = validate(test_subject());
                    let control = RigidBody::default()
                        .with_shape(Cuboid::cube(1.0));
                    let rigid_body = RigidBody::default()
                        .with_shape(Cuboid::cube(1.0))
                        .with_rotation(rot.into())
                        .with_translation_vect(2.0 * offset.as_vect());

                    let result = detection.compute_contacts(&handle(control), &handle(rigid_body));

                    assert!(result.is_none());
                }

                quickcheck::quickcheck(property as fn(inputs::UnitVect, inputs::UnitQuat));
            }

            #[test]
            fn it_handles_vertex_vertex_collisions() {
                let mut detection = validate(test_subject());
                let control = RigidBody::default()
                    .with_shape(Cuboid::new(1.0, 2.0, 1.0));
                let rigid_body = RigidBody::default()
                    .with_shape(Cuboid::new(2.0, 1.0, 1.0))
                    .with_translation(1.49, 1.49, 0.99);

                let result = detection.compute_contacts(&handle(control), &handle(rigid_body));

                assert!(result.is_some());

                let contact_event = result.unwrap();

                assert_eq!(contact_event.points().len(), 1);
                // TODO officially support vertex – vertex contacts
                // assert_approx_eq!(contact_event.normal(), Vect::new(1.0, 0.0, 0.0));
                // assert_approx_eq!(contact_event.points().first(), Vect::new(0.5, 0.0, 0.0));
            }

            #[test]
            fn it_handles_edge_edge_collisions() {
                let mut detection = validate(test_subject());
                let control = RigidBody::default()
                    .with_shape(Cuboid::new(1.0, 2.0, 3.0));
                let rigid_body = RigidBody::default()
                    .with_shape(Cuboid::new(1.0, 2.0, 3.0))
                    .with_translation(0.99, 1.49, 0.00);

                let result = detection.compute_contacts(&handle(control), &handle(rigid_body));

                assert!(result.is_some());

                let contact_event = result.unwrap();

                assert_eq!(contact_event.points().len(), 1);
                // TODO officially support edge - edge contacts
                // assert_approx_eq!(contact_event.points(0), Vect::new(0.5, 0.0, 0.0));
                // assert_approx_eq!(contact_event.normal(), Vect::new(1.0, 0.0, 0.0));
            }

            #[test]
            fn it_handles_face_face_collisions() {
                let mut detection = validate(test_subject());
                let control = RigidBody::default()
                    .with_shape(Cuboid::cube(1.0));
                let rigid_body = RigidBody::default()
                    .with_shape(Cuboid::cube(1.0))
                    .with_translation(0.99, 0.50, 0.50);

                let result = detection.compute_contacts(&handle(control), &handle(rigid_body));

                assert!(result.is_some());

                let contact_event = result.unwrap();

                // TODO officially support face - face contacts
                // assert_eq!(contact_event.points().len(), 4);
                assert_approx_eq!(*contact_event.normal(), Vect::new(-1.0, 0.0, 0.0));
            }

            #[test]
            fn it_handles_edge_face_collisions() {
                let mut detection = validate(test_subject());
                let control = RigidBody::default()
                    .with_shape(Cuboid::cube(1.0));
                let rigid_body = RigidBody::default()
                    .with_shape(Cuboid::cube(1.0))
                    .with_translation(0.49 + 0.5*(2.0 as Scalar).sqrt(), 0.00, 0.00)
                    .with_axis_angle(Vect::new(0.0, 0.0, 1.0), PI/4.0);

                let result = detection.compute_contacts(&handle(control), &handle(rigid_body));

                assert!(result.is_some());

                let contact_event = result.unwrap();

                // TODO officially support edge - face contacts
                // assert_eq!(contact_event.points().len(), 2);
                assert_approx_eq!(*contact_event.normal(), Vect::new(-1.0, 0.0, 0.0));
            }

            #[test]
            fn it_handles_vertex_face_collisions() {
                let mut detection = validate(test_subject());
                let control = RigidBody::default()
                    .with_shape(Cuboid::cube(1.0));
                let initial_axis = Vect::new(1.0, 1.0, 1.0).normalize();
                let final_axis = Vect::new(1.0, 0.0, 0.0);
                let rotation = initial_axis.cross(final_axis);
                let rigid_body = RigidBody::default()
                    .with_shape(Cuboid::cube(1.0))
                    .with_translation((0.98 + (3.0 as Scalar).sqrt())/2.0, 0.1, 0.0)
                    .with_axis_angle(rotation, rotation.length().asin());

                let result = detection.compute_contacts(&handle(control), &handle(rigid_body));

                assert!(result.is_some());

                let contact_event = result.unwrap();

                assert_eq!(contact_event.points().len(), 1);
                assert_approx_eq!(*contact_event.normal(), Vect::new(-1.0, 0.0, 0.0));
                assert_approx_eq!(*contact_event.point(0), Vect::new(0.495, 0.1, 0.0));
            }

            #[test]
            fn it_always_has_the_normal_pointing_towards_the_first_body() {
                fn property(offset: inputs::UnitVect, rot: inputs::UnitQuat) {
                    let mut detection = validate(test_subject());
                    let control = RigidBody::default()
                        .with_shape(Cuboid::cube(1.0));
                    let rigid_body = RigidBody::default()
                        .with_shape(Cuboid::cube(1.0))
                        .with_rotation(rot.into())
                        .with_translation_vect(0.45 * offset.as_vect());
                    let control_handle = handle(control);
                    let rigid_body_handle = handle(rigid_body);

                    let contact_event = detection.compute_contacts(&control_handle, &rigid_body_handle)
                        .expect("Test was setup to always have an intersection, but that didn't happen");

                    let projection = {
                        let control_obj = control_handle.borrow();
                        let body_obj = rigid_body_handle.borrow();

                        contact_event.normal()
                            .dot(control_obj.translation() - body_obj.translation())
                    };

                    assert!(projection > 0.0, format!("Expected the projected relative distance in the direction of the normal to always be positive, but got {}", projection));
                }

                quickcheck::quickcheck(property as fn(inputs::UnitVect, inputs::UnitQuat));
            }

            fn handle<B: Body + 'static>(body: B) -> BodyHandle {
                BodyHandle::new(Box::new(body))
            }
        }
    };
}