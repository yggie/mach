macro_rules! assert_integrator_behaviour(
    { $( $lines:item )+ } => (
        $( $lines )+

        mod integrator_behaviour {
            use super::test_subject;

            use maths::{IntegratableMut, Integrator, Motion, Quat, Transform, Vect};

            fn validate<I: Integrator>(input: I) -> I {
                input
            }

            #[test]
            fn it_can_correctly_integrate_zero_motion() {
                let integrator = validate(test_subject());
                let mut transform = Transform::identity();
                let mut motion = Motion::stationary();

                {
                    let mut integratable = IntegratableMut::new(&mut transform, &mut motion);

                    integrator.integrate_in_place(&mut integratable, 0.5, Vect::zero());
                }

                assert_approx_eq!(transform.translation, Vect::zero());
                assert_approx_eq!(transform.rotation, Quat::identity());
                assert_approx_eq!(motion.velocity, Vect::zero());
                assert_approx_eq!(motion.angular_velocity, Vect::zero());
            }

            #[test]
            fn it_does_not_change_velocity_or_angular_velocity_with_no_applied_force() {
                let integrator = validate(test_subject());
                let mut transform = Transform::identity();
                let mut motion = Motion::stationary()
                    .with_velocity(1.0, 0.0, 0.0)
                    .with_angular_velocity(0.0, 1.0, 0.0);

                {
                    let mut integratable = IntegratableMut::new(&mut transform, &mut motion);

                    integrator.integrate_in_place(&mut integratable, 0.5, Vect::zero());
                    integrator.integrate_in_place(&mut integratable, 0.5, Vect::zero());
                }

                assert_approx_eq!(motion.velocity, Vect::new(1.0, 0.0, 0.0));
                assert_approx_eq!(motion.angular_velocity, Vect::new(0.0, 1.0, 0.0));
            }

            #[test]
            fn it_correctly_integrates_simple_constant_velocity_linear_motion() {
                let integrator = validate(test_subject());
                let mut transform = Transform::identity();
                let mut motion = Motion::stationary()
                    .with_velocity(1.0, 0.0, 0.0);

                {
                    let mut integratable = IntegratableMut::new(&mut transform, &mut motion);

                    integrator.integrate_in_place(&mut integratable, 0.5, Vect::zero());
                }

                assert_approx_eq!(transform.translation, Vect::new(0.5, 0.0, 0.0));
                assert_approx_eq!(motion.velocity, Vect::new(1.0, 0.0, 0.0));
            }

            #[test]
            fn it_correctly_integrates_simple_constant_force_linear_motion() {
                let integrator = validate(test_subject());
                let mut transform = Transform::identity();
                let mut motion = Motion::stationary()
                    .with_velocity(1.0, 0.0, 0.0);

                {
                    let mut integratable = IntegratableMut::new(&mut transform, &mut motion);

                    integrator.integrate_in_place(&mut integratable, 0.5, Vect::new(1.0, 0.0, 0.0));
                }

                assert_approx_eq!(transform.translation.normalize(), Vect::new(1.0, 0.0, 0.0));
                assert_approx_eq!(motion.velocity, Vect::new(1.5, 0.0, 0.0));
            }
        }
    );
);