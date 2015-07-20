use core::Handle;
use math::Vector;

/// `ContactPair` holds information about the identities of the intersecting
/// bodies.
#[derive(Clone, Copy)]
pub enum ContactPair<H: Handle> {
    /// Represents a pair of bodies which both act as rigid bodies.
    RigidRigid(H, H),
    /// Represents a pair of bodies in which one of them acts as a rigid body,
    /// and the other acts as a static body.
    RigidStatic {
        /// The `ID` for the rigid body.
        rigid_id: H,
        /// The `ID` for the static body.
        static_id: H
    },
}

/// `Contact` holds contact information for two intersecting bodies.
#[derive(Clone, Copy)]
pub struct Contact<H: Handle> {
    /// The pair of identifiers along with type information associated with the
    /// contact.
    pub ids: ContactPair<H>,
    /// The center of the contact.
    pub center: Vector,
    /// The surface normal of the contact.
    pub normal: Vector
}
