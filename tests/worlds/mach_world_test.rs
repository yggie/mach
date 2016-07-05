assert_world_behaviour! {
    use MachWorld;
    use collisions::Narrowphase;

    pub fn test_subject<T>() -> MachWorld<T> {
        MachWorld::new()
    }
}
