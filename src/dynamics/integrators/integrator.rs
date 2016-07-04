use Scalar;
use maths::Vec3D;
use dynamics::Integratable;

pub trait Integrator {
    fn integrate_in_place(&self, target: Integratable, time_step: Scalar, linear_acceleration: Vec3D);
}
