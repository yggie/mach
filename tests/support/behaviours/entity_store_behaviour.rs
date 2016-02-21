macro_rules! assert_entity_store_behaviour {
    { $( $lines:item )+ } => {
        $( $lines )+

        mod entity_store_behaviour {
            use super::test_subject;

            use utils::EntityBuilder;
            use entities::EntityStore;

            fn validate<S: EntityStore>(input: S) -> S {
                input
            }

            #[test]
            fn it_can_create_rigid_bodies() {
                let mut store = validate(test_subject());

                let id = {
                    let builder = EntityBuilder::from_store(&mut store);
                    builder.create_rigid_body()
                };

                let _body = store.find_body(id)
                    .expect("expected to find the body recently created, but got nothing");
            }
        }
    };
}
