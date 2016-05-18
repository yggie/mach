assert_narrowphase_behaviour! {
    use collisions::narrowphase::NullNarrowphase;

    pub fn test_subject() -> NullNarrowphase {
        NullNarrowphase::new()
    }
}
