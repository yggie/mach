use math::Vector;
use shapes::Sphere;
use properties::Rigid;
use core::{ Body, State };
use integrators::StateIntegrator;

pub fn single_step_constant_velocity_zero_rotation_test(integrator: StateIntegrator) {
    let mut b = Body::new(box Sphere::new(1.0), box Rigid::new(1.0), State::new_stationary().with_velocity(1.0, -1.0, 0.5));

    integrator(&mut b, 0.3);

    assert_eq!(b.position(), Vector::new(0.30, -0.30, 0.15));
}

pub fn single_step_constant_force_zero_rotation_test(integrator: StateIntegrator) {
    // allow tolerance for different integration techniques
    let tolerance = 0.10;

    let mut b = Body::new(box Sphere::new(1.0), box Rigid::new(1.0), State::new_stationary().with_velocity(1.0, -1.0, 0.5));
    b.impulse.set(2.5, -2.5, 3.3);

    integrator(&mut b, 0.2);

    let diff = b.position() - Vector::new(0.30, -0.30, 0.15);
    if diff.length() > tolerance {
        panic!("Expected {} to be less than {}", diff.length(), tolerance);
    }

    let diff_vel = b.velocity() - Vector::new(1.50, -1.50, 1.16);
    if diff_vel.length() > tolerance {
        panic!("Velocity change failed tolerance test: {} should be less than {}",
             diff_vel.length(), tolerance);
    }
}

macro_rules! assert_time_integrator_behaviour(
    ($integrator:ident) => (
        #[test]
        fn single_step_constant_velocity_zero_rotation_test() {
            behaviours::single_step_constant_velocity_zero_rotation_test($integrator);
        }

        #[test]
        fn single_step_constant_force_zero_rotation_test() {
            behaviours::single_step_constant_force_zero_rotation_test($integrator);
        }
    );
)
