macro_rules! assert_collision_space_behaviour(
    { $( $lines:item )+ } => (

        $( $lines )+

        mod collision_space_behaviour {
            use super::test_subject;

            use mach::{EntityDesc, ID};
            use mach::entities::RigidBody;
            use mach::collisions::CollisionSpace;

            fn validate<C: CollisionSpace>(input: C) -> C {
                input
            }

            #[test]
            pub fn it_can_create_rigid_bodies() {
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
            pub fn it_can_find_a_rigid_body_by_id() {
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
            pub fn it_can_modify_a_rigid_body_by_id() {
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
            pub fn it_can_iterate_over_all_rigid_bodies() {
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
            pub fn it_can_mutate_all_bodies() {
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
        }
    );
);
