assert_collision_object_space_behaviour! {
    use collisions::MachCollisionObjectSpace;

    pub fn test_subject<T>() -> MachCollisionObjectSpace<T> {
        MachCollisionObjectSpace::new()
    }
}
