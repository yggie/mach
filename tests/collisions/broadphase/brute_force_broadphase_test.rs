assert_new_broadphase_behaviour! {
    use collisions::{BruteForceBroadphase, NarrowphaseData};

    pub fn test_subject<T>() -> BruteForceBroadphase<T> where T: NarrowphaseData {
        BruteForceBroadphase::new()
    }
}
