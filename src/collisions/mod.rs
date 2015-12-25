//! The `collisions` subsystem is responsible for the static behaviour of the
//! physics engine. It contains subcomponents to handle storage, retrieval and
//! queries for physical bodies.

mod gjkepa;
mod contact;
mod intersection;
mod simple_collision_space;

/// TODO temporary workaround for the issue of rexporting traits, see https://github.com/rust-lang/rust/issues/16264
pub mod collision_space;
/// TODO temporary workaround for the issue of rexporting traits, see https://github.com/rust-lang/rust/issues/16264
pub mod contact_detector;

pub use self::gjkepa::ContactCache;
pub use self::contact::{Contact, ContactPair};
pub use self::intersection::Intersection;
pub use self::collision_space::CollisionSpace;
pub use self::contact_detector::ContactDetector;
pub use self::simple_collision_space::SimpleCollisionSpace;
