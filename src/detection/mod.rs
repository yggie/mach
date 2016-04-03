//! The `collisions` subsystem is responsible for the static behaviour of the
//! physics engine. It contains subcomponents to handle storage, retrieval and
//! queries for physical bodies.

mod gjkepa;
mod contact_set;
mod contact_event;
mod gjk_epa_detection;

/// TODO temporary workaround for the issue of rexporting traits, see https://github.com/rust-lang/rust/issues/16264
pub mod detection;

pub use self::gjkepa::ContactCache;
pub use self::detection::Detection;
pub use self::contact_set::ContactSet;
pub use self::contact_event::ContactEvent;
pub use self::gjk_epa_detection::GjkEpaDetection;
