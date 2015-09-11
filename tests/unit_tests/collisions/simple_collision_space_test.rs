assert_collision_space_behaviour! {
    use mach::collisions::SimpleCollisionSpace;

    pub fn test_subject() -> SimpleCollisionSpace {
        SimpleCollisionSpace::new()
    }
}
