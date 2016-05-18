macro_rules! assert_narrowphase_behaviour {
    ( $( $lines:item )+ ) => {
        $( $lines )+

        mod narrowphase_behaviour {
            use super::test_subject;

            use maths::Transform;
            use shapes::Cuboid;
            use collisions::{CollisionData, Narrowphase};

            #[test]
            fn it_passes_the_collision_test_for_definitely_intersecting_bodies() {
                let mut narrowphase = validate(test_subject());
                let shape_0 = Box::new(Cuboid::cube(1.0));
                let shape_1 = Box::new(Cuboid::cube(1.0));
                let transform_0 = Transform::identity();
                let transform_1 = Transform::identity();

                let mut data_0 = CollisionData::new(&mut narrowphase, shape_0, transform_0);
                let mut data_1 = CollisionData::new(&mut narrowphase, shape_1, transform_1);

                narrowphase.update(&mut data_0);
                narrowphase.update(&mut data_1);

                assert!(
                    narrowphase.check(&data_0, &data_1),
                    "expected the intersecting bodies to return a positive collision test, but did not"
                );
            }

            fn validate<NS: Narrowphase>(input: NS) -> NS {
                input
            }
        }
    };
}
