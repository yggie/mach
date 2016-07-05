macro_rules! assert_narrowphase_behaviour {
    ( $( $lines:item )+ ) => {
        $( $lines )+

        mod narrowphase_behaviour {
            use std::mem;
            use std::marker::PhantomData;

            use super::type_marker;

            use shapes::Cuboid;
            use collisions::{BodyData, BodyDef, Narrowphase};

            #[test]
            fn it_passes_the_collision_test_for_definitely_intersecting_bodies() {
                let marker = type_marker();

                let mut body_0 = create_body_data(0, marker, BodyDef {
                    shape: Box::new(Cuboid::cube(1.0)),
                    .. BodyDef::default()
                });
                let mut body_1 = create_body_data(0, marker, BodyDef {
                    shape: Box::new(Cuboid::cube(1.0)),
                    .. BodyDef::default()
                });

                Narrowphase::update(&mut body_0);
                Narrowphase::update(&mut body_1);

                assert!(
                    Narrowphase::test(&body_0, &body_1),
                    "expected the intersecting bodies to return a positive collision test, but did not"
                );
            }

            fn create_body_data<N>(id: u32, _marker: PhantomData<N>, def: BodyDef) -> BodyData<N> where N: Narrowphase {
                BodyData::new(unsafe { mem::transmute(id) }, def)
            }
        }
    };
}
