assert_collision_object_space_behaviour! {
    use collisions::{MachCollisionObjectSpace, NarrowphaseData};

    pub fn test_subject<T>() -> MachCollisionObjectSpace<T> where T: NarrowphaseData {
        MachCollisionObjectSpace::new()
    }
}
