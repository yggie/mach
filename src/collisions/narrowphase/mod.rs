//! This module encapsulates the responsibilities associated with collision
//! detection at a microscopic level, where exact details of the intersections
//! can be obtained.

mod intersection;
mod gjk_epa;

/// TODO temporary workaround for the issue of rexporting traits, see https://github.com/rust-lang/rust/issues/16264
pub mod narrowphase;

pub use self::gjk_epa::GjkEpa;
pub use self::narrowphase::NarrowPhase;
pub use self::intersection::Intersection;
