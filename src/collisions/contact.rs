use core::UID;
use maths::Vector;

/// `Constraint` contains information regarding a constraint in a physical
/// system.
#[derive(Clone, Copy)]
pub enum Constraint {
    /// Represents a pair of bodies which both act as rigid bodies.
    RigidRigid(UID, UID),
    /// Represents a pair of bodies in which one of them acts as a rigid body,
    /// and the other acts as a static body.
    RigidStatic {
        /// The `UID` for the rigid body.
        rigid_id: UID,
        /// The `UID` for the static body.
        static_id: UID
    },
}

/// `Contact` holds contact information for two intersecting bodies.
#[derive(Clone, Copy)]
pub struct Contact {
    /// The pair of identifiers along with type information associated with the
    /// contact.
    pub constraint: Constraint,
    /// The center of the contact.
    pub center: Vector,
    /// The surface normal of the contact.
    pub normal: Vector
}
