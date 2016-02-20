macro_rules! assert_integrator_behaviour(
    { $( $lines:item )+ } => (
        $( $lines )+

        mod integrator_behaviour {
            use std::cell::{Ref, RefCell};

            use super::test_subject;

            use maths::{IntegratableMut, Integrator, Quat, Vect};
            use utils::StandaloneEntityBuilder;
            use entities::{Body, BodyType, RigidBody};

            fn validate<I: Integrator>(input: I) -> I {
                input
            }

            #[test]
            fn it_can_correctly_integrate_zero_motion() {
                let integrator = validate(test_subject());
                let cell = StandaloneEntityBuilder::cube(1.0)
                    .with_translation(0.0, 0.0, 0.0)
                    .with_no_rotation()
                    .with_velocity(0.0, 0.0, 0.0)
                    .with_angular_velocity(0.0, 0.0, 0.0)
                    .build_cell();

                {
                    let mut integratable = cell.build_integratable_mut();

                    integrator.integrate_in_place(&mut integratable, 0.5, Vect::zero());
                }

                let body = cell.borrow();
                let rigid_body = body.to_rigid_body();

                assert_approx_eq!(rigid_body.translation(), Vect::zero());
                assert_approx_eq!(rigid_body.rotation(), Quat::identity());
                assert_approx_eq!(rigid_body.velocity(), Vect::zero());
                assert_approx_eq!(rigid_body.angular_velocity(), Vect::zero());
            }

            #[test]
            fn it_does_not_change_velocity_or_angular_velocity_with_no_applied_force() {
                let integrator = validate(test_subject());
                let cell = StandaloneEntityBuilder::cube(1.0)
                    .with_translation(0.0, 0.0, 0.0)
                    .with_no_rotation()
                    .with_velocity(1.0, 0.0, 0.0)
                    .with_angular_velocity(0.0, 1.0, 0.0)
                    .build_cell();

                {
                    let mut integratable = cell.build_integratable_mut();

                    integrator.integrate_in_place(&mut integratable, 0.5, Vect::zero());
                    integrator.integrate_in_place(&mut integratable, 0.5, Vect::zero());
                }

                let body = cell.borrow();
                let rigid_body = body.to_rigid_body();

                assert_approx_eq!(rigid_body.velocity(), Vect::new(1.0, 0.0, 0.0));
                assert_approx_eq!(rigid_body.angular_velocity(), Vect::new(0.0, 1.0, 0.0));
            }

            #[test]
            fn it_correctly_integrates_simple_constant_velocity_linear_motion() {
                let integrator = validate(test_subject());
                let cell = StandaloneEntityBuilder::cube(1.0)
                    .with_translation(0.0, 0.0, 0.0)
                    .with_velocity(1.0, 0.0, 0.0)
                    .build_cell();

                {
                    let mut integratable = cell.build_integratable_mut();

                    integrator.integrate_in_place(&mut integratable, 0.5, Vect::zero());
                }

                let body = cell.borrow();
                let rigid_body = body.to_rigid_body();

                assert_approx_eq!(rigid_body.translation(), Vect::new(0.5, 0.0, 0.0));
                assert_approx_eq!(rigid_body.velocity(), Vect::new(1.0, 0.0, 0.0));
            }

            #[test]
            fn it_correctly_integrates_simple_constant_force_linear_motion() {
                let integrator = validate(test_subject());
                let cell = StandaloneEntityBuilder::cube(1.0)
                    .with_translation(0.0, 0.0, 0.0)
                    .with_velocity(1.0, 0.0, 0.0)
                    .build_cell();

                {
                    let mut integratable = cell.build_integratable_mut();

                    integrator.integrate_in_place(&mut integratable, 0.5, Vect::new(1.0, 0.0, 0.0));
                }

                let body = cell.borrow();
                let rigid_body = body.to_rigid_body();

                assert_approx_eq!(rigid_body.translation().normalize(), Vect::new(1.0, 0.0, 0.0));
                assert_approx_eq!(rigid_body.velocity(), Vect::new(1.5, 0.0, 0.0));
            }

            trait BuildIntegratableMut {
                fn build_integratable_mut(&self) -> IntegratableMut;
            }

            impl BuildIntegratableMut for RefCell<Box<Body>> {
                fn build_integratable_mut(&self) -> IntegratableMut {
                    IntegratableMut::new(self.borrow_mut())
                }
            }

            trait ToRigidBody {
                fn to_rigid_body(&self) -> &RigidBody;
            }

            impl<'a> ToRigidBody for Ref<'a, Box<Body>> {
                fn to_rigid_body(&self) -> &RigidBody {
                    return match self.downcast() {
                        BodyType::Rigid(rigid_body) => rigid_body,

                        _otherwise => panic!("Incorrect test setup! Non-rigid body found"),
                    };
                }
            }

            trait BuildCell {
                fn build_cell(self) -> RefCell<Box<Body>>;
            }

            impl BuildCell for StandaloneEntityBuilder {
                fn build_cell(self) -> RefCell<Box<Body>> {
                    let rigid_body = self.build_rigid_body();

                    return RefCell::new(Box::new(rigid_body));
                }
            }
        }
    );
);
