use core::{ RigidBody, SharedCell, StaticBody };
use maths::Vector;

/// `Constraint` contains information regarding a constraint in a physical
/// system.
pub enum Constraint {
    /// A contact constraint between two `RigidBody` instances.
    RigidRigid {
        /// The shared reference to the contacting bodies.
        rigid_body_cells: (SharedCell<RigidBody>, SharedCell<RigidBody>),
        /// The center of the contact.
        contact_center: Vector,
        /// The surface normal of the contact.
        contact_normal: Vector,
    },

    /// A contact constraint between a `RigidBody` and a `StaticBody`.
    RigidStatic {
        /// A shared reference to the `RigidBody`.
        rigid_body_cell: SharedCell<RigidBody>,
        /// A shared reference to the `StaticBody`.
        static_body_cell: SharedCell<StaticBody>,
        /// The center of the contact.
        contact_center: Vector,
        /// The surface normal of the contact.
        contact_normal: Vector,
    },
}
