assert_world_behaviour! {
    use MachWorld;

    pub fn test_subject<T>() -> MachWorld<T> {
        MachWorld::new()
    }
}
