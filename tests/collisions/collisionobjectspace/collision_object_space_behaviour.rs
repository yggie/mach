macro_rules! assert_collision_object_space_behaviour {
    { $( $lines:item )+ } => {
        $( $lines )+

        mod collision_object_space_behaviour {
            extern crate quickcheck;

            use ID;
            use utils::Handle;
            use collisions::{BodyDef, CollisionBody, CollisionGroup, CollisionObjectSpace, Narrowphase};

            use tests::support::TestBody;
            use super::test_subject;

            #[test]
            fn it_generates_unique_ids() {
                let mut space = validate(test_subject());

                let id_0 = id(space.create_body(BodyDef {
                    group: CollisionGroup::Default,
                    .. BodyDef::default()
                }, ()));

                let id_1 = id(space.create_body(BodyDef {
                    group: CollisionGroup::Environment,
                    .. BodyDef::default()
                }, ()));

                let id_2 = id(space.create_body(BodyDef {
                    group: CollisionGroup::Default,
                    .. BodyDef::default()
                }, ()));

                assert!(id_0 != id_1);
                assert!(id_0 != id_2);
                assert!(id_1 != id_2);
            }

            #[test]
            fn it_only_considers_non_environment_bodies_as_foreground() {
                let mut space = validate(test_subject());

                space.create_body(BodyDef {
                    group: CollisionGroup::Default,
                    .. BodyDef::default()
                }, ());

                let count = space.foreground_bodies_iter().count();
                assert_eq!(count, 1);

                space.create_body(BodyDef {
                    group: CollisionGroup::Environment,
                    .. BodyDef::default()
                }, ());

                let count = space.foreground_bodies_iter().count();
                assert_eq!(count, 1);

                space.create_body(BodyDef {
                    group: CollisionGroup::A,
                    .. BodyDef::default()
                }, ());

                let count = space.foreground_bodies_iter().count();
                assert_eq!(count, 2);
            }

            fn validate<T>(input: T) -> T where T: CollisionObjectSpace<TestBody> {
                input
            }

            fn id(handle: Handle<TestBody>) -> ID {
                let borrowed = handle.borrow();

                return borrowed.id();
            }
        }
    };
}
