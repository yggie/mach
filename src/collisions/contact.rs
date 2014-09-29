use bodies::Body;
use math::Vector;

use std::rc::Rc;

/// Represents a point of contact between two physical entities. Holds
/// references to the contacting bodies and a point of contact.
pub struct Contact<'a> {
    /// References to the contacting bodies.
    pub bodies: [Rc<Body<'a>>, ..2],
    /// The point of contact.
    pub point: Vector,
}

impl<'a> Contact<'a> {

    /// Constructs a Contact object using the given references and a point of
    /// contact.
    pub fn new<'a>(a: Rc<Body>, b: Rc<Body>, point: Vector) -> Contact<'a> {
        Contact{ bodies: [a, b], point: point }
    }
}
