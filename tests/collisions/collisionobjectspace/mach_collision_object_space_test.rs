assert_collision_object_space_behaviour! {
    use collisions::{CollisionObject, MachCollisionObjectSpace};

    pub fn test_subject<O>() -> MachCollisionObjectSpace<O> where O: CollisionObject {
        MachCollisionObjectSpace::new()
    }
}
