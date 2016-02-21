macro_rules! assert_detection_behaviour {
    ( $( $lines:item )+ ) => {
        $( $lines )+

        mod detection_behaviour {
            use super::test_subject;

            use entities::MachStore;
            use detection::Detection;

            fn validate<D: Detection>(input: D) -> D {
                input
            }

            // TODO move tests from space_behaviour and the like into this file
            #[test]
            fn it_will_eventually_work() {
                let store = MachStore::new();
                let _detection = validate(test_subject(&store));
                // do nothing, but it will be fine
            }
        }
    };
}
