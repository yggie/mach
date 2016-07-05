assert_narrowphase_behaviour! {
    use collisions::narrowphase::NullNarrowphase;

    use std::marker::PhantomData;

    pub fn type_marker() -> PhantomData<NullNarrowphase> {
        PhantomData
    }
}
