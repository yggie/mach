#[macro_use]
#[cfg(test)]
#[path="../../../tests/collisions/detection/detection_behaviour.rs"]
mod behaviours;

mod contact;
mod contact_set;

pub mod gjkepa;
/// TODO temporary workaround for the issue of rexporting traits, see https://github.com/rust-lang/rust/issues/16264
pub mod detection;

pub use self::gjkepa::GJKEPADetection;
pub use self::contact::Contact;
pub use self::contact_set::ContactSet;
pub use self::detection::Detection;
