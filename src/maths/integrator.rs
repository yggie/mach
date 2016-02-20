use Scalar;
use maths::{IntegratableMut, Vect};

pub trait Integrator {
    fn integrate_in_place(&self, integratable: &mut IntegratableMut, time_step: Scalar, applied_force: Vect);
}
