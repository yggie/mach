// #[cfg(test)]
// #[path="../../../tests/maths/integrators/semi_implicit_euler_test.rs"]
// mod tests;
//
use Scalar;
use maths::{Quat, Vec3D};
use dynamics::{Integratable, Integrator};

/// An implementation of the Semi-Implicit Euler integration strategy.
pub struct SemiImplicitEuler;

impl SemiImplicitEuler {
    pub fn new() -> SemiImplicitEuler {
        SemiImplicitEuler
    }
}

impl Integrator for SemiImplicitEuler {
    fn integrate_in_place(&self, mut target: Integratable, t: Scalar, linear_acceleration: Vec3D) {
        *target.velocity_mut() += linear_acceleration * t;
        *target.translation_mut() += target.velocity() * t;

        let w = target.angular_velocity().clone();
        let w_as_quat = Quat::new(0.0, w.x * t, w.y * t, w.z * t);
        let q = Quat::from(target.rotation().clone());
        *target.rotation_mut() = (q + w_as_quat * q * 0.5).normalize();
    }
}
