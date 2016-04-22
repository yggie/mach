use Scalar;
use maths::{IntegratableMut, Vec3D};

pub trait Integrator {
    fn integrate_in_place(&self, integratable: &mut IntegratableMut, time_step: Scalar, linear_acceleration: Vec3D);
}
