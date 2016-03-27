macro_rules! assert_entity_store_behaviour {
    { $( $lines:item )+ } => {
        $( $lines )+

        mod entity_store_behaviour {
            use super::test_subject;

            use ID;
            use shapes::Cuboid;
            use entities::{EntityStore, RigidBody};

            #[test]
            fn it_can_create_rigid_bodies() {
                let mut store = validate(test_subject());

                let id = store.add_rigid_body(RigidBody::default());

                // TODO assert properties of the resulting body?
                let _body = store.find_body(id)
                    .expect("expected to find the body recently created, but got nothing");
            }

            // #[test]
            // fn it_can_iterate_over_all_rigid_bodies() {
            //     let mut store = validate(test_subject());
            //     let entity_desc = BodyParams::cube(1.0)
            //         .with_mass(3.0)
            //         .as_stationary();
            //
            //     let ids = vec!(
            //         space.create_rigid_body(&entity_desc),
            //         space.create_rigid_body(&entity_desc),
            //         space.create_rigid_body(&entity_desc),
            //     );
            //
            //     let iterated_ids: Vec<ID> = space.rigid_bodies_iter()
            //         .map(|body| body.id())
            //         .collect();
            //
            //     assert_ids_match(ids, iterated_ids);
            // }

            #[test]
            fn it_can_mutate_all_bodies() {
                let mut store = validate(test_subject());

                let prototype = RigidBody::default()
                    .with_shape(Cuboid::cube(1.0))
                    .with_mass(3.0);
                let ids = vec![
                    store.add_rigid_body(prototype.clone()),
                    store.add_rigid_body(prototype.clone()),
                    store.add_rigid_body(prototype.clone()),
                ];

                let iterated_ids: Vec<ID> = store.rigid_body_iter_mut()
                    .map(|body| body.id())
                    .collect();

                assert_ids_match(ids, iterated_ids);
            }

            fn validate<S: EntityStore>(input: S) -> S {
                input
            }

            fn assert_ids_match(mut ids_0: Vec<ID>, mut ids_1: Vec<ID>) {
                ids_0.sort_by(|a, b| a.cmp(&b));
                ids_1.sort_by(|a, b| a.cmp(&b));

                for (id, expected_id) in ids_0.iter().zip(ids_1.iter()) {
                    assert_eq!(id, expected_id);
                }
            }
        }
    };
}
