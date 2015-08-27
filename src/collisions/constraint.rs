use core::UID;
use maths::Vector;

/// `Constraint` contains information regarding a constraint in a physical
/// system.
#[derive(Clone, Copy)]
pub enum Constraint {
    /// Represents a pair of bodies which both act as rigid bodies.
    RigidRigid {
        /// The `UID`s for the contacting rigid bodies.
        uids: (UID, UID),
        /// The center of the contact.
        contact_center: Vector,
        /// The surface normal of the contact.
        contact_normal: Vector
    },

    /// Represents a pair of bodies in which one of them acts as a rigid body,
    /// and the other acts as a static body.
    RigidStatic {
        /// The `UID` for the rigid body.
        rigid_uid: UID,
        /// The `UID` for the static body.
        static_uid: UID,
        /// The center of the contact.
        contact_center: Vector,
        /// The surface normal of the contact.
        contact_normal: Vector
    },
}
