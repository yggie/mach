macro_rules! assert_collision_space_behaviour(
    { $( $lines:item )+ } => (

        $( $lines )+

        mod collision_space_behaviour {
            use super::test_subject;

            use mach::{EntityDesc, ID, PI, Scalar};
            use mach::maths::{ApproxEq, Vector};
            use mach::entities::RigidBody;
            use mach::collisions::{CollisionSpace, ContactPair};

            fn validate<C: CollisionSpace>(input: C) -> C {
                input
            }

            struct ExpectedContact {
                ids: (ID, ID),
                point: Vector,
                normal: Vector,
            }

            fn assert_contact_points<C: CollisionSpace>(collision_space: &mut C, expected_contacts: Vec<ExpectedContact>) {
                let contacts_option = collision_space.find_contacts();
                if contacts_option.is_some() && expected_contacts.len() == 0 {
                    panic!("Expected not to find any contacts, but found some");
                } else if contacts_option.is_none() && expected_contacts.len() > 0 {
                    panic!("Expected to find contacts, but found none");
                }

                match expected_contacts.len() {
                    0 => (),

                    1 => {
                        let contacts = contacts_option
                            .expect("Expected to find contacts, but found none");

                        assert!(contacts.len() == 1, "Expected to find only one contact, but found more");

                        let contact = &contacts[0];
                        let expected_contact = &expected_contacts[0];

                        let ids = match &contact.pair {
                            &ContactPair::RigidRigid(ref body_0, ref body_1) => {
                                (body_0.borrow().id(), body_1.borrow().id())
                            },

                            _otherwise => unimplemented!(),
                        };

                        if (ids.0 != expected_contact.ids.0 && ids.1 != expected_contact.ids.1) && (ids.0 != expected_contact.ids.1 && ids.1 != expected_contact.ids.0) {
                            panic!("Expected contact to be between bodies {:?} but was instead between bodies {:?}", expected_contact.ids, ids);
                        }

                        if !ApproxEq::approx_eq(contact.normal, expected_contact.normal) {
                            panic!("Expected contact normal to equal {:?} but got {:?} instead", expected_contact.normal, contact.normal);
                        }

                        if !ApproxEq::approx_eq(contact.center, expected_contact.point) {
                            panic!("Expected contact point to equal {:?} but got {:?} instead", expected_contact.point, contact.center);
                        }
                    },

                    _otherwise => unimplemented!(),
                }
            }

            #[test]
            fn it_can_create_rigid_bodies() {
                let mut collision_space = validate(test_subject());
                let entity_desc = EntityDesc::default()
                    .as_cube(1.0)
                    .with_density(3.0)
                    .as_stationary();

                let id = collision_space.create_body(&entity_desc);

                // TODO assertions about rigid bodies count?

                let rigid_body = collision_space.find_body(id)
                    .expect("expected to find the rigid body recently created but got nothing");

                // TODO assertions about shape?
                // TODO fix this!
                // assert_eq!(rigid_body.mass(), entity_desc.material.mass_of(&shape));
                assert_eq!(rigid_body.coefficient_of_restitution(), entity_desc.material.coefficient_of_restitution());
            }

            #[test]
            fn it_can_find_a_rigid_body_by_id() {
                let mut collision_space = validate(test_subject());
                let entity_desc = EntityDesc::default()
                    .as_cube(1.0)
                    .with_mass(3.0)
                    .as_stationary();

                collision_space.create_body(&entity_desc);
                let id = collision_space.create_body(&entity_desc);
                collision_space.create_body(&entity_desc);

                let body: &RigidBody = &collision_space.find_body(id)
                    .expect("expected to find the rigid body recently created but got nothing");

                assert_eq!(body.id(), id);
            }

            #[test]
            fn it_can_modify_a_rigid_body_by_id() {
                let mut collision_space = validate(test_subject());
                let entity_desc = EntityDesc::default()
                    .as_cube(1.0)
                    .with_mass(3.0)
                    .as_stationary();

                let id = collision_space.create_body(&entity_desc);
                collision_space.create_body(&entity_desc);
                collision_space.create_body(&entity_desc);

                let body: &mut RigidBody = &mut collision_space.find_body_mut(id)
                    .expect("expected to find the rigid body recently created but got nothing");

                assert_eq!(body.id(), id);
            }

            #[test]
            fn it_can_iterate_over_all_rigid_bodies() {
                let mut collision_space = validate(test_subject());
                let entity_desc = EntityDesc::default()
                    .as_cube(1.0)
                    .with_mass(3.0)
                    .as_stationary();

                let mut ids = vec!(
                    collision_space.create_body(&entity_desc),
                    collision_space.create_body(&entity_desc),
                    collision_space.create_body(&entity_desc),
                );

                let mut iterated_ids: Vec<ID> = collision_space.bodies_iter()
                    .map(|body| body.id())
                    .collect();

                ids.sort_by(|a, b| a.cmp(&b));
                iterated_ids.sort_by(|a, b| a.cmp(&b));
                for (id, expected_id) in iterated_ids.iter().zip(ids.iter()) {
                    assert_eq!(id, expected_id);
                }
            }

            #[test]
            fn it_can_mutate_all_bodies() {
                let mut collision_space = validate(test_subject());
                let entity_desc = EntityDesc::default()
                    .as_cube(1.0)
                    .with_mass(3.0)
                    .as_stationary();

                let mut ids = vec!(
                    collision_space.create_body(&entity_desc),
                    collision_space.create_body(&entity_desc),
                    collision_space.create_body(&entity_desc),
                );

                let mut iterated_ids: Vec<ID> = collision_space.bodies_iter_mut()
                    .map(|mut body| (&mut body as &mut RigidBody).id())
                    .collect();

                ids.sort_by(|a, b| a.cmp(&b));
                iterated_ids.sort_by(|a, b| a.cmp(&b));

                for (id, expected_id) in iterated_ids.iter().zip(ids.iter()) {
                    assert_eq!(id, expected_id);
                }
            }

            #[test]
            fn it_correctly_identifies_non_colliding_vertex_vertex_cases() {
                let mut collision_space = validate(test_subject());
                let params_0 = EntityDesc::default()
                    .as_cuboid(1.0, 2.0, 1.0);
                let params_1 = EntityDesc::default()
                    .as_cuboid(2.0, 1.0, 1.0)
                    .with_pos(1.51, 1.51, 1.01);

                collision_space.create_body(&params_0);
                collision_space.create_body(&params_1);

                assert_contact_points(&mut collision_space, vec!());
            }

            #[test]
            fn it_correctly_identifies_colliding_vertex_vertex_cases() {
                let mut collision_space = validate(test_subject());
                let params_0 = EntityDesc::default()
                    .as_cuboid(1.0, 2.0, 1.0);
                let params_1 = EntityDesc::default()
                    .as_cuboid(2.0, 1.0, 1.0)
                    .with_pos(1.49, 1.49, 0.99);

                collision_space.create_body(&params_0);
                collision_space.create_body(&params_1);

                let contacts = collision_space.find_contacts()
                    .expect("Expected a collision, but did not find any");

                assert_eq!(contacts.len(), 1);
                // TODO officially support vertex – vertex contacts
                // let contact = contacts.first().expect("Expected at least one contact");
                // assert_eq!(contact.normal, Vector::new(1.0, 0.0, 0.0));
                // assert_eq!(contact.point, Vector::new(0.5, 0.0, 0.0));
            }

            #[test]
            fn it_correctly_identifies_non_colliding_edge_edge_cases() {
                let mut collision_space = validate(test_subject());
                let params_0 = EntityDesc::default()
                    .as_cuboid(1.0, 2.0, 3.0);
                let params_1 = EntityDesc::default()
                    .as_cuboid(1.0, 2.0, 3.0)
                    .with_pos(1.01, 1.51, 0.00);

                collision_space.create_body(&params_0);
                collision_space.create_body(&params_1);

                assert_contact_points(&mut collision_space, vec!());
            }

            #[test]
            fn it_correctly_identifies_colliding_edge_edge_cases() {
                let mut collision_space = validate(test_subject());
                let params_0 = EntityDesc::default()
                    .as_cuboid(1.0, 2.0, 3.0);
                let params_1 = EntityDesc::default()
                    .as_cuboid(1.0, 2.0, 3.0)
                    .with_pos(0.99, 1.49, 0.00);

                collision_space.create_body(&params_0);
                collision_space.create_body(&params_1);

                let contacts = collision_space.find_contacts()
                    .expect("Expected a collision, but did not find any");

                assert_eq!(contacts.len(), 1);
                // TODO officially support edge - edge contacts
                // let contact = contacts.first().expect("Expected at least one contact");
                // assert!(contact.point.distance_to(Vector::new(0.5, 0.0, 0.0)) < 0.001);
                // assert!(contact.normal.distance_to(Vector::new(1.0, 0.0, 0.0)) < 0.001);
            }

            #[test]
            fn it_correctly_identifies_non_colliding_face_face_cases() {
                let mut collision_space = validate(test_subject());
                let params_0 = EntityDesc::default()
                    .as_cuboid(1.0, 1.0, 1.0);
                let params_1 = EntityDesc::default()
                    .as_cuboid(1.0, 1.0, 1.0)
                    .with_pos(1.01, 0.50, 0.50);

                collision_space.create_body(&params_0);
                collision_space.create_body(&params_1);

                assert_contact_points(&mut collision_space, vec!());
            }

            #[test]
            fn it_correctly_identifies_colliding_face_face_cases() {
                let mut collision_space = validate(test_subject());
                let params_0 = EntityDesc::default()
                    .as_cuboid(1.0, 1.0, 1.0);
                let params_1 = EntityDesc::default()
                    .as_cuboid(1.0, 1.0, 1.0)
                    .with_pos(0.99, 0.50, 0.50);

                collision_space.create_body(&params_0);
                collision_space.create_body(&params_1);

                let contacts = collision_space.find_contacts()
                    .expect("Expected a collision, but did not find any");

                assert_eq!(contacts.len(), 1);
                let contact = contacts.first().expect("Expected at least one contact");
                assert_eq!(contact.normal, Vector::new(1.0, 0.0, 0.0));
                // TODO officially support face - face contacts
                // assert_eq!(contact.point, Vector::new(0.995, 0.750, 0.750));
            }

            #[test]
            fn it_correctly_identifies_non_colliding_edge_face_cases() {
                let mut collision_space = validate(test_subject());
                let params_0 = EntityDesc::default()
                    .as_cuboid(1.0, 1.0, 1.0);
                let params_1 = EntityDesc::default()
                    .as_cuboid(1.0, 1.0, 1.0)
                    .with_pos(0.51 + 0.5*(2.0 as Scalar).sqrt(), 0.00, 0.00)
                    .with_axis_angle(Vector::new(0.0, 1.0, 0.0), PI/4.0);

                collision_space.create_body(&params_0);
                collision_space.create_body(&params_1);

                assert_contact_points(&mut collision_space, vec!());
            }

            #[test]
            fn it_correctly_identifies_colliding_edge_face_cases() {
                let mut collision_space = validate(test_subject());
                let params_0 = EntityDesc::default()
                    .as_cuboid(1.0, 1.0, 1.0);
                let params_1 = EntityDesc::default()
                    .as_cuboid(1.0, 1.0, 1.0)
                    .with_pos(0.49 + 0.5*(2.0 as Scalar).sqrt(), 0.00, 0.00)
                    .with_axis_angle(Vector::new(0.0, 0.0, 1.0), PI/4.0);

                collision_space.create_body(&params_0);
                collision_space.create_body(&params_1);

                let contacts = collision_space.find_contacts()
                    .expect("Expected a collision, but did not find any");

                assert_eq!(contacts.len(), 1);
                let contact = contacts.first().expect("Expected at least one contact");
                assert_eq!(contact.normal, Vector::new(1.0, 0.0, 0.0));
                // TODO officially support edge - face contacts
                // assert_eq!(contact.point, Vector::new(0.5, 0.0, 0.0));
            }

            #[test]
            fn it_correctly_identifies_non_colliding_vertex_face_cases() {
                let mut collision_space = validate(test_subject());
                let params_0 = EntityDesc::default()
                    .as_cuboid(1.0, 1.0, 1.0);

                let initial_axis = Vector::new(1.0, 1.0, 1.0).normalize();
                let final_axis = Vector::new(1.0, 0.0, 0.0);
                let rotation = initial_axis.cross(final_axis);
                let params_1 = EntityDesc::default()
                    .as_cuboid(1.0, 1.0, 1.0)
                    .with_pos((1.01 + (3.0 as Scalar).sqrt())/2.0, 0.0, 0.0)
                    .with_axis_angle(rotation, rotation.length().asin());

                collision_space.create_body(&params_0);
                collision_space.create_body(&params_1);

                assert_contact_points(&mut collision_space, vec!());
            }

            #[test]
            fn it_correctly_identifies_colliding_vertex_face_cases() {
                let mut collision_space = validate(test_subject());
                let params_0 = EntityDesc::default()
                    .as_cuboid(1.0, 1.0, 1.0);

                let initial_axis = Vector::new(1.0, 1.0, 1.0).normalize();
                let final_axis = Vector::new(1.0, 0.0, 0.0);
                let rotation = initial_axis.cross(final_axis);
                let params_1 = EntityDesc::default()
                    .as_cuboid(1.0, 1.0, 1.0)
                    .with_pos((0.98 + (3.0 as Scalar).sqrt())/2.0, 0.1, 0.0)
                    .with_axis_angle(rotation, rotation.length().asin());

                let id_0 = collision_space.create_body(&params_0);
                let id_1 = collision_space.create_body(&params_1);

                assert_contact_points(&mut collision_space, vec!(
                    ExpectedContact {
                        ids: (id_0, id_1),
                        point: Vector::new(0.495, 0.1, 0.0),
                        normal: Vector::new(1.0, 0.0, 0.0),
                    }
                ));
            }
        }
    );
);
