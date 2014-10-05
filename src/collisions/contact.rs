use core::UID;
use math::Vector;

/// Represents a point of contact between two physical entities. Holds
/// references to the contacting bodies and a point of contact.
pub struct Contact<'a> {
    /// References to the two bodies.
    pub body_ids: [UID, ..2],
    /// The point of contact.
    pub point: Vector,
    /// The contact normal.
    pub normal: Vector,
}
