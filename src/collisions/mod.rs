//! The `collisions` subsystem is responsible for the static behaviour of the
//! physics engine. It contains subcomponents to handle storage, retrieval and
//! queries for physical bodies.

mod contact;
mod simple_collision_space;

pub mod epa;
pub mod gjk;
pub mod narrowphase;
/// TODO temporary workaround for the issue of rexporting traits, see https://github.com/rust-lang/rust/issues/16264
pub mod collision_space;

pub use self::contact::{Contact, ContactPair};
pub use self::collision_space::CollisionSpace;
pub use self::simple_collision_space::SimpleCollisionSpace;
