use Float;
use maths::{ State, Quat, Vector };
use dynamics::Integrator;

/// An implementation of the Semi-Implicit Euler integration strategy.
pub struct SemiImplicitEuler;

impl SemiImplicitEuler {
    /// Integrates the `State` in place for the given time step and applied
    /// force.
    pub fn integrate_in_place(&self, state: &mut State, time_step: Float, applied_force: Vector) {
        // TODO deal with temporaries once language limitation is resolved, see https://github.com/rust-lang/rfcs/pull/396
        let t = time_step;
        let p = state.pos();
        let v = state.vel();
        let new_velocity = v + applied_force * t;
        state.set_vel(&new_velocity);
        state.set_pos(&(p + new_velocity * t));

        let w = state.ang_vel();
        let w_as_quat = Quat::new(0.0, w.x * t, w.y * t, w.z * t);
        let q = state.rot();
        let new_rotation = q + w_as_quat * q * 0.5;

        state.set_rot(&new_rotation.normalize());
    }
}

impl Integrator for SemiImplicitEuler {
    #[inline]
    fn integrate_in_place(&self, state: &mut State, time_step: Float, applied_force: Vector) {
        (self as &SemiImplicitEuler).integrate_in_place(state, time_step, applied_force);
    }
}
