assert_dynamics_behaviour! {
    use mach::dynamics::SimpleDynamics;

    pub fn test_subject() -> SimpleDynamics {
        SimpleDynamics::new()
    }
}
