#[cfg(test)]
#[path="../../../tests/maths/integrators/semi_implicit_euler_test.rs"]
mod tests;

use Scalar;
use maths::{IntegratableMut, Integrator, Quat, Vect};

/// An implementation of the Semi-Implicit Euler integration strategy.
pub struct SemiImplicitEuler;

impl SemiImplicitEuler {
    pub fn new() -> SemiImplicitEuler {
        SemiImplicitEuler
    }
}

impl Integrator for SemiImplicitEuler {
    fn integrate_in_place(&self, integratable: &mut IntegratableMut, time_step: Scalar, linear_acceleration: Vect) {
        let t = time_step;

        // TODO replace with AddAssign once stabilized: https://github.com/rust-lang/rust/issues/28235
        *integratable.velocity_mut() = integratable.velocity() + linear_acceleration * t;
        // TODO replace with AddAssign once stabilized: https://github.com/rust-lang/rust/issues/28235
        *integratable.translation_mut() = integratable.translation() + integratable.velocity() * t;

        let w = integratable.angular_velocity().clone();
        let w_as_quat = Quat::new(0.0, w.x * t, w.y * t, w.z * t);
        let q = integratable.rotation().clone();
        *integratable.rotation_mut() = (q + w_as_quat * q * 0.5).normalize();
    }
}
