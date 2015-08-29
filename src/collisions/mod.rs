//! The `collisions` subsystem is responsible for the static behaviour of the
//! physics engine. It contains subcomponents to handle storage, retrieval and
//! queries for physical bodies.

mod constraint;
mod simple_collisions;

pub mod narrowphase;
/// TODO temporary workaround for the issue of rexporting traits, see https://github.com/rust-lang/rust/issues/16264
pub mod collision_space;

pub use self::constraint::Constraint;
pub use self::narrowphase::NarrowPhase;
pub use self::collision_space::CollisionSpace;
pub use self::simple_collisions::SimpleCollisions;
