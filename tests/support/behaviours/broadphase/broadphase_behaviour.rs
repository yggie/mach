// TODO remove new_ prefix
macro_rules! assert_new_broadphase_behaviour {
    { $( $lines:item )+ } => {
        $( $lines )+

        mod broadphase_behaviour {
            extern crate quickcheck;

            use super::test_subject;

            use tests::support::assert_properties_for_actions;
            use tests::support::behaviours::broadphase::{BroadphaseAction, NoBackgroundObjectCollisionsProperty};

            #[test]
            fn it_behaves_like_a_broadphase() {
                fn property(actions: Vec<BroadphaseAction>) {
                    let properties = vec!(
                        NoBackgroundObjectCollisionsProperty::new(),
                    );
                    let broadphase = test_subject();

                    assert_properties! {
                        target: broadphase,
                        actions: actions,
                        properties: &properties,
                    }
                }

                quickcheck::quickcheck(property as fn(Vec<BroadphaseAction>));
            }
        }
    };
}
