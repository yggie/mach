use std::collections::HashMap;

use maths::Vector;
use core::{ Body, UID };

/// This data structure maintains the accumulated impulse acting on a `Body`.
pub struct ForceAccumulator(HashMap<UID, (Vector, Vector)>);

impl ForceAccumulator {
    /// Instantiates a new empty `ForceAccumulator`.
    pub fn new() -> ForceAccumulator {
        ForceAccumulator(HashMap::new())
    }

    /// Computes the total force and torque acting on a `Body` and stores the
    /// result.
    pub fn add_impulse(&mut self, body: &Body, impulse: Vector, point: Vector) {
        let &(force, torque) = self.0.get(&body.id())
            .unwrap_or(&(Vector::new_zero(), Vector::new_zero()));

        let new_force = force + impulse;
        let new_torque = torque + (point - body.position()).cross(impulse);

        self.0.insert(body.id(), (new_force, new_torque));
    }

    /// Retrieves the forces acting on the `Body` and resets the stored values.
    pub fn consume_forces(&mut self, body: &Body) -> (Vector, Vector) {
        self.0.remove(&body.id()).unwrap_or((Vector::new_zero(), Vector::new_zero()))
    }
}
