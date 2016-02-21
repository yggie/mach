assert_narrowphase_behaviour! {
    use narrowphase::BruteForce;

    pub fn test_subject() -> BruteForce {
        BruteForce::new()
    }
}
