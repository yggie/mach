macro_rules! assert_narrowphase_behaviour {
    ( $( $lines:item )+ ) => {
        $( $lines )+

        mod narrowphase_behaviour {
            use super::test_subject;

            use utils::EntityBuilder;
            use entities::{EntityStore, MachStore};
            use narrowphase::Narrowphase;

            fn validate<N: Narrowphase>(input: N) -> N {
                input
            }

            #[test]
            fn it_passes_the_collision_test_for_intersecting_bodies() {
                let mut store = MachStore::new();
                let mut narrowphase = validate(test_subject(&store));
                let prototype = EntityBuilder::from_store(&mut store)
                    .as_cube(1.0)
                    .with_translation(0.0, 0.0, 0.0);

                let id_0 = prototype.clone().create_rigid_body();
                let id_1 = prototype.clone().create_rigid_body();

                assert!(
                    narrowphase.test(id_0, id_1),
                    "expected the intersecting bodies to return a positive collision test, but did not"
                );
            }
        }
    };
}
