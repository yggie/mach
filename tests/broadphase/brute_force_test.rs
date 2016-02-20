assert_broadphase_behaviour! {
    use broadphase::BruteForce;
    use entities::EntityStore;

    pub fn test_subject<ES: EntityStore>(_store: &ES) -> BruteForce<ES> {
        BruteForce::new()
    }
}
