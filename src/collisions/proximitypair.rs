use collisions::Contact;
use bodies::Body;
use math::Vector;

use std::rc::Rc;

/// Represents two bodies in close proximity.
pub struct ProximityPair {
    /// References to the two bodies.
    pub bodies: [Rc<Body>, ..2],
}

impl ProximityPair {

    /// Constructs a new ProximityPair object from two bodies.
    pub fn new(a: Rc<Body>, b: Rc<Body>) -> ProximityPair {
        ProximityPair{ bodies: [a, b] }
    }

    /// Conditionally executes the given function if a contact is present.
    pub fn if_contact(&self, callback: |Contact|) {
        callback(Contact::new(self.bodies[0].clone(), self.bodies[1].clone(), Vector::zero()));
    }
}
