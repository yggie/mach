macro_rules! assert_broadphase_behaviour {
    { $( $lines:item )+ } => {
        $( $lines )+

        mod broadphase_behaviour {
            extern crate quickcheck;

            use super::test_subject;

            use collisions::{BodyDef, Broadphase, CollisionGroup, CollisionObjectSpace};

            use tests::support::TestBody;

            #[test]
            fn it_does_not_allow_environment_bodies_to_collide() {
                let mut broadphase = validate(test_subject());

                broadphase.create_body(BodyDef {
                    group: CollisionGroup::Environment,
                    .. BodyDef::default()
                }, ());

                broadphase.create_body(BodyDef {
                    group: CollisionGroup::Environment,
                    .. BodyDef::default()
                }, ());

                let count = broadphase.close_proximity_pairs_iter().count();
                assert!(count == 0, "expected no bodies to be in contact, but found a few");
            }

            #[test]
            fn it_generates_close_proximity_pairs_for_intersecting_bodies() {
                let mut broadphase = validate(test_subject());
                let def = BodyDef::default();

                let count = count_pairs_after_adding(&mut broadphase, def.clone());
                assert_eq!(count, 0);

                let count = count_pairs_after_adding(&mut broadphase, def.clone());
                assert_eq!(count, 1);

                let count = count_pairs_after_adding(&mut broadphase, def.clone());
                assert_eq!(count, 3);

                let count = count_pairs_after_adding(&mut broadphase, def.clone());
                assert_eq!(count, 6);
            }

            #[test]
            fn it_does_not_generate_close_proximity_pairs_for_intersecting_environment_bodies() {
                let mut broadphase = validate(test_subject());
                let def = BodyDef {
                    group: CollisionGroup::Environment,
                    .. BodyDef::default()
                };

                let count = count_pairs_after_adding(&mut broadphase, def.clone());
                assert_eq!(count, 0);

                let count = count_pairs_after_adding(&mut broadphase, def.clone());
                assert_eq!(count, 0);

                let count = count_pairs_after_adding(&mut broadphase, BodyDef {
                    group: CollisionGroup::Default,
                    .. def.clone()
                });
                assert_eq!(count, 2);
            }

            fn validate<B>(input: B) -> B where B: Broadphase<TestBody> {
                input
            }

            fn count_pairs_after_adding<B>(broadphase: &mut B, def: BodyDef) -> usize where B: Broadphase<TestBody> {
                broadphase.create_body(def, ());

                return broadphase.close_proximity_pairs_iter().count();
            }
        }
    };
}
