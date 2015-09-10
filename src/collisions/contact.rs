use core::SharedCell;
use maths::Vector;
use entities::{ RigidBody, StaticBody };

/// Encapsulates the different possible pairs of physical bodies.
pub enum ContactPair {
    /// A pair of contacting `RigidBody` instances.
    RigidRigid(SharedCell<RigidBody>, SharedCell<RigidBody>),
    /// A pair consisting of a `RigidBody` and a `StaticBody`.
    RigidStatic(SharedCell<RigidBody>, SharedCell<StaticBody>),
}

/// `Contact` contains information regarding contact between two physical
/// entities.
pub struct Contact {
    /// The pair of contacting bodies.
    pub pair: ContactPair,
    /// The center of the contact.
    pub center: Vector,
    /// The surface normal of the contact.
    pub normal: Vector,
}
