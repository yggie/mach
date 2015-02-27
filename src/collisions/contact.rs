use math::Vector;

/// `Contact` holds contact information for two intersecting bodies.
#[derive(Copy)]
pub struct Contact<T> {
    /// References to the two bodies.
    pub body_ids: [T; 2],
    /// The center of the contact.
    pub center: Vector,
    /// The contact normal.
    pub normal: Vector,
}
