assert_collision_object_space_behaviour! {
    use collisions::{CollisionBody, MachCollisionObjectSpace};

    pub fn test_subject<B>() -> MachCollisionObjectSpace<B> where B: CollisionBody {
        MachCollisionObjectSpace::new()
    }
}
