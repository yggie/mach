assert_new_broadphase_behaviour! {
    use collisions::BruteForceBroadphase;

    pub fn test_subject<T>() -> BruteForceBroadphase<T> {
        BruteForceBroadphase::new()
    }
}
