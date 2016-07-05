assert_collision_object_space_behaviour! {
    use collisions::{MachCollisionObjectSpace, Narrowphase};

    pub fn test_subject<N>() -> MachCollisionObjectSpace<N, ()> where N: Narrowphase {
        MachCollisionObjectSpace::new()
    }
}
