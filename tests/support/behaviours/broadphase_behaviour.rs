macro_rules! assert_broadphase_behaviour {
    { $( $lines:item )+ } => {
        $( $lines )+

        mod broadphase_behaviour {
            use super::test_subject;

            use shapes::Cuboid;
            use entities::{EntityStore, MachStore, RigidBody, StaticBody};
            use broadphase::Broadphase;

            fn validate<B: Broadphase>(input: B) -> B {
                input
            }

            #[test]
            fn it_generates_contact_candidates_for_intersecting_rigid_bodies() {
                let mut store = MachStore::new();
                let mut broadphase = validate(test_subject(&store));
                let prototype = RigidBody::default()
                    .with_shape(Cuboid::cube(1.0))
                    .with_zero_translation();

                let count = count_after_adding(&mut store, &mut broadphase, prototype.clone());
                assert_eq!(count, 0);

                let count = count_after_adding(&mut store, &mut broadphase, prototype.clone());
                assert_eq!(count, 1);

                let count = count_after_adding(&mut store, &mut broadphase, prototype.clone());
                assert_eq!(count, 3);

                let count = count_after_adding(&mut store, &mut broadphase, prototype.clone());
                assert_eq!(count, 6);
            }

            #[test]
            fn it_does_not_generate_contact_candidates_for_intersecting_static_bodies() {
                let mut store = MachStore::new();
                let mut broadphase = validate(test_subject(&store));
                let static_body_prototype = StaticBody::default()
                    .with_shape(Cuboid::cube(1.0))
                    .with_translation(0.0, 0.0, 0.0);
                let rigid_body_prototype = RigidBody::default()
                    .with_shape(Cuboid::cube(1.0))
                    .with_translation(0.0, 0.0, 0.0);

                let count = count_after_adding_static(&mut store, &mut broadphase, static_body_prototype.clone());
                assert_eq!(count, 0);

                let count = count_after_adding_static(&mut store, &mut broadphase, static_body_prototype.clone());
                assert_eq!(count, 0);

                let count = count_after_adding(&mut store, &mut broadphase, rigid_body_prototype.clone());
                assert_eq!(count, 2);
            }

            fn count_after_adding<E: EntityStore, B: Broadphase<EntityStore=E>>(store: &mut E, broadphase: &mut B, rigid_body: RigidBody) -> usize {
                let id = store.add_rigid_body(rigid_body);
                let handle = store.find_body_handle(id).unwrap();
                broadphase.notify_body_created(&store, handle);

                return broadphase.contact_candidate_pairs_iter(&store).count();
            }

            fn count_after_adding_static<E: EntityStore, B: Broadphase<EntityStore=E>>(store: &mut E, broadphase: &mut B, static_body: StaticBody) -> usize {
                let id = store.add_static_body(static_body);
                let handle = store.find_body_handle(id).unwrap();
                broadphase.notify_body_created(&store, handle);

                return broadphase.contact_candidate_pairs_iter(&store).count();
            }
        }
    };
}
