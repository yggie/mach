use Scalar;
use maths::Vect;
use entities::Moveable;

/// This trait should be implemented by objects with the ability to integrate a
/// `State` over time.
pub trait Integrator {
    /// Integrates the `State` object in place for the given time step and
    /// applied force.
    fn integrate(&self, &mut Moveable, Scalar, Vect);
}
