macro_rules! assert_collision_object_space_behaviour {
    { $( $lines:item )+ } => {
        $( $lines )+

        mod collision_object_space_behaviour {
            extern crate quickcheck;

            use super::test_subject;

            use tests::support::assert_properties_for_actions;
            use tests::support::behaviours::collisionobjectspace::{CollisionObjectSpaceAction, ForegroundObjectCountProperty, UniqueIDProperty};

            #[test]
            fn it_behaves_like_a_collision_object_space() {
                fn property(actions: Vec<CollisionObjectSpaceAction>) {
                    let properties = vec!(
                        ForegroundObjectCountProperty::new(),
                        UniqueIDProperty::new(),
                    );
                    let object_space = test_subject();

                    assert_properties! {
                        target: object_space,
                        actions: actions,
                        properties: &properties,
                    }
                }

                quickcheck::quickcheck(property as fn(Vec<CollisionObjectSpaceAction>));
            }
        }
    };
}
