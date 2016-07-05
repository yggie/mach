assert_broadphase_behaviour! {
    use collisions::Narrowphase;
    use collisions::broadphase::BruteForceBroadphase;

    pub fn test_subject<N, T>() -> BruteForceBroadphase<N, T> where N: Narrowphase {
        BruteForceBroadphase::new()
    }
}
