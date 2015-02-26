use core::UID;
use math::Vector;

/// `Contact` holds contact information for two intersecting bodies.
#[derive(Clone, Copy)]
pub struct Contact {
    /// References to the two bodies.
    pub body_ids: [UID; 2],
    /// The center of the contact.
    pub center: Vector,
    /// The contact normal.
    pub normal: Vector,
}
