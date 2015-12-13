use Scalar;
use maths::{State, Vector};

/// This trait should be implemented by objects with the ability to integrate a
/// `State` over time.
pub trait Integrator {
    /// Integrates the `State` object in place for the given time step and
    /// applied force.
    fn integrate_in_place(&self, &mut State, Scalar, Vector);

    /// Integrates the `State` object for the given time step and applied
    /// force then returns the new `State`. The default implementation uses
    /// `integrate_in_place` internally.
    #[inline]
    fn integrate(&self, state: State, time_step: Scalar, applied_force: Vector) -> State {
        let mut new_state = state.clone();
        self.integrate_in_place(&mut new_state, time_step, applied_force);

        return new_state;
    }
}
