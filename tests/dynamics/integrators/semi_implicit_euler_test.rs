assert_integrator_behaviour! {
    use dynamics::integrators::SemiImplicitEuler;

    pub fn test_subject() -> SemiImplicitEuler {
        SemiImplicitEuler
    }
}
