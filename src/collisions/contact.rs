use bodies::Body;
use math::Vector;

use std::rc::Rc;

/// Represents a point of contact between two physical entities. Holds
/// references to the contacting bodies and a point of contact.
pub struct Contact<'a> {
    /// References to the contacting bodies. The order of the references matter,
    /// the first body is always assumed to be in the direction of the contact
    /// normal.
    pub bodies: [Rc<Body<'a>>, ..2],
    /// The point of contact.
    pub point: Vector,
    /// The contact normal.
    pub normal: Vector,
}
