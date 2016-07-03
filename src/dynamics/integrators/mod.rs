mod integratable;
mod semi_implicit_euler;

pub use self::integratable::Integratable;
pub use self::semi_implicit_euler::SemiImplicitEuler;

use Scalar;
use maths::{Motion, Transform, Vec3D};

pub trait Integrator {
    fn integrate_in_place(&self, target: Integratable, time_step: Scalar, linear_acceleration: Vec3D);
}
