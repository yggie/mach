assert_integrator_behaviour! {
    use maths::integrators::SemiImplicitEuler;

    pub fn test_subject() -> SemiImplicitEuler {
        SemiImplicitEuler
    }
}
