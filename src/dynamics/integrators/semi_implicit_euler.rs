use core::Float;
use maths::{ State, Quaternion, Vector };
use dynamics::Integrator;

/// An implementation of the Semi-Implicit Euler integration strategy.
pub struct SemiImplicitEuler;

impl SemiImplicitEuler {
    /// Integrates the `State` in place for the given time step and applied
    /// force.
    pub fn integrate_in_place(&self, state: &mut State, time_step: Float, applied_force: Vector) {
        // TODO deal with temporaries once language limitation is resolved, see https://github.com/rust-lang/rfcs/pull/396
        let t = time_step;
        let p = state.position();
        let v = state.velocity();
        let new_velocity = v + applied_force * t;
        state.set_velocity_with_vector(new_velocity);
        state.set_position_with_vector(p + new_velocity * t);

        let w = state.angular_velocity();
        let w_as_quat = Quaternion::new(0.0, w[0] * t, w[1] * t, w[2] * t);
        let q = state.rotation();
        let new_rotation = q + w_as_quat * q * 0.5;

        state.set_rotation(new_rotation.normalize());
    }
}

impl Integrator for SemiImplicitEuler {
    #[inline]
    fn integrate_in_place(&self, state: &mut State, time_step: Float, applied_force: Vector) {
        (self as &SemiImplicitEuler).integrate_in_place(state, time_step, applied_force);
    }
}
