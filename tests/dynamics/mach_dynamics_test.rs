assert_dynamics_behaviour! {
    use mach::dynamics::MachDynamics;

    pub fn test_subject() -> MachDynamics {
        MachDynamics::new()
    }
}
