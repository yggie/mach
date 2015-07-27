use std::collections::HashMap;

use maths::Vector;
use core::{ Body, Handle };

/// This data structure maintains the accumulated impulse acting on a `Body`.
pub struct ForceAccumulator<H: Handle>(HashMap<H, (Vector, Vector)>);

impl<H: Handle> ForceAccumulator<H> {
    /// Instantiates a new empty `ForceAccumulator`.
    pub fn new() -> ForceAccumulator<H> {
        ForceAccumulator(HashMap::new())
    }

    /// Computes the total force and torque acting on a `Body` and stores the
    /// result.
    pub fn add_impulse(&mut self, body: &Body<H>, impulse: Vector, point: Vector) {
        let &(force, torque) = self.0.get(&body.id())
            .unwrap_or(&(Vector::new_zero(), Vector::new_zero()));

        let new_force = force + impulse;
        let new_torque = torque - impulse.cross(point - body.position());

        self.0.insert(body.id(), (new_force, new_torque));
    }

    /// Retrieves the forces acting on the `Body` and resets the stored values.
    pub fn consume_forces(&mut self, body: &Body<H>) -> (Vector, Vector) {
        self.0.remove(&body.id()).unwrap_or((Vector::new_zero(), Vector::new_zero()))
    }
}
