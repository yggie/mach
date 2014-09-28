use bodies::Body;
use math::Vector;

use std::rc::Rc;

/// Represents a point of contact between two physical entities. Holds
/// references to the contacting bodies and a point of contact.
pub struct Contact {
    /// References to the contacting bodies.
    pub bodies: [Rc<Body>, ..2],
    /// The point of contact.
    pub point: Vector,
}

impl Contact {

    /// Constructs a Contact object using the given references and a point of
    /// contact.
    pub fn new(a: Rc<Body>, b: Rc<Body>, point: Vector) -> Contact {
        Contact{ bodies: [a, b], point: point }
    }
}
