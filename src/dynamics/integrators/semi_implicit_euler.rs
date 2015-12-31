use Scalar;
use maths::{Quat, Vect};
use dynamics::Integrator;
use entities::Moveable;

/// An implementation of the Semi-Implicit Euler integration strategy.
pub struct SemiImplicitEuler;

impl Integrator for SemiImplicitEuler {
    fn integrate(&self, object: &mut Moveable, time_step: Scalar, applied_force: Vect) {
        let t = time_step;

        // TODO replace with AddAssign once stabilized: https://github.com/rust-lang/rust/issues/28235
        *object.velocity_mut() = object.velocity() + applied_force * t;
        // TODO replace with AddAssign once stabilized: https://github.com/rust-lang/rust/issues/28235
        *object.translation_mut() = object.translation() + object.velocity() * t;

        let w = object.angular_velocity().clone();
        let w_as_quat = Quat::new(0.0, w.x * t, w.y * t, w.z * t);
        let q = object.rotation().clone();
        *object.rotation_mut() = (q + w_as_quat * q * 0.5).normalize();
    }
}
