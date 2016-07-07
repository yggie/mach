assert_broadphase_behaviour! {
    use collisions::CollisionBody;
    use collisions::broadphase::BruteForceBroadphase;

    pub fn test_subject<B>() -> BruteForceBroadphase<B> where B: CollisionBody {
        BruteForceBroadphase::new()
    }
}
