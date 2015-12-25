assert_space_behaviour! {
    use mach::detection::MachSpace;

    pub fn test_subject() -> MachSpace {
        MachSpace::new()
    }
}
