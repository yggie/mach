macro_rules! assert_collision_space_behaviour(
    { $( $lines:item )+ } => (

        $( $lines )+

        mod collision_space_behaviour {
            use super::test_subject;

            use mach::core::{ State, UID };
            use mach::shapes::Cuboid;
            use mach::entities::{ Material, RigidBody };
            use mach::collisions::CollisionSpace;

            fn validate<C: CollisionSpace>(input: C) -> C {
                input
            }

            #[test]
            pub fn it_can_create_rigid_bodies() {
                let mut collision_space = validate(test_subject());
                let shape = Cuboid::new_cube(1.0);
                let material = &Material::new_with_density(3.0);

                let uid = collision_space.create_body(shape.clone(), material, State::new_stationary());

                // TODO assertions about rigid bodies count?

                let rigid_body = collision_space.find_body(uid)
                    .expect("expected to find the rigid body recently created but got nothing");

                // TODO assertions about shape?
                assert_eq!(rigid_body.mass(), material.mass_of(&shape));
                assert_eq!(rigid_body.coefficient_of_restitution(), material.coefficient_of_restitution());
            }

            #[test]
            pub fn it_can_find_a_rigid_body_by_id() {
                let mut collision_space = validate(test_subject());
                let shape = Cuboid::new_cube(1.0);
                let material = &Material::new_with_mass(3.0);
                let state = State::new_stationary();
                collision_space.create_body(shape.clone(), material, state);
                let uid = collision_space.create_body(shape.clone(), material, state);
                collision_space.create_body(shape.clone(), material, state);

                let body: &RigidBody = &collision_space.find_body(uid)
                    .expect("expected to find the rigid body recently created but got nothing");

                assert_eq!(body.id(), uid);
            }

            #[test]
            pub fn it_can_modify_a_rigid_body_by_id() {
                let mut collision_space = validate(test_subject());
                let shape = Cuboid::new_cube(1.0);
                let material = &Material::new_with_density(3.0);
                let state = State::new_stationary();
                let uid = collision_space.create_body(shape.clone(), material, state);
                collision_space.create_body(shape.clone(), material, state);
                collision_space.create_body(shape.clone(), material, state);

                let body: &mut RigidBody = &mut collision_space.find_body_mut(uid)
                    .expect("expected to find the rigid body recently created but got nothing");

                assert_eq!(body.id(), uid);
            }

            #[test]
            pub fn it_can_iterate_over_all_rigid_bodies() {
                let mut collision_space = validate(test_subject());
                let shape = Cuboid::new_cube(1.0);
                let material = &Material::new_with_mass(3.0);
                let mut uids = vec!(
                    collision_space.create_body(shape.clone(), material, State::new_stationary()),
                    collision_space.create_body(shape.clone(), material, State::new_stationary()),
                    collision_space.create_body(shape.clone(), material, State::new_stationary()),
                );

                let mut iterated_uids: Vec<UID> = collision_space.bodies_iter()
                    .map(|body| body.id())
                    .collect();

                uids.sort_by(|a, b| a.cmp(&b));
                iterated_uids.sort_by(|a, b| a.cmp(&b));
                for (uid, expected_uid) in iterated_uids.iter().zip(uids.iter()) {
                    assert_eq!(uid, expected_uid);
                }
            }

            #[test]
            pub fn it_can_mutate_all_bodies() {
                let mut collision_space = validate(test_subject());
                let shape = Cuboid::new_cube(1.0);
                let material = &Material::new_with_density(3.0);
                let mut uids = vec!(
                    collision_space.create_body(shape.clone(), material, State::new_stationary()),
                    collision_space.create_body(shape.clone(), material, State::new_stationary()),
                    collision_space.create_body(shape.clone(), material, State::new_stationary()),
                );

                let mut iterated_uids: Vec<UID> = collision_space.bodies_iter_mut()
                    .map(|mut body| (&mut body as &mut RigidBody).id())
                    .collect();

                uids.sort_by(|a, b| a.cmp(&b));
                iterated_uids.sort_by(|a, b| a.cmp(&b));

                for (uid, expected_uid) in iterated_uids.iter().zip(uids.iter()) {
                    assert_eq!(uid, expected_uid);
                }
            }
        }
    );
);