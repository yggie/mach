assert_broadphase_behaviour! {
    use collisions::CollisionObject;
    use collisions::broadphase::BruteForceBroadphase;

    pub fn test_subject<O>() -> BruteForceBroadphase<O> where O: CollisionObject {
        BruteForceBroadphase::new()
    }
}
