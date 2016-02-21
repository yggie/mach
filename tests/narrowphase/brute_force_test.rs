assert_narrowphase_behaviour! {
    use entities::EntityStore;
    use narrowphase::BruteForce;

    pub fn test_subject<ES: EntityStore>(_store: &ES) -> BruteForce<ES> {
        BruteForce::new()
    }
}
