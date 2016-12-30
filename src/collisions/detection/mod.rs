#[macro_use]
#[cfg(test)]
#[path="../../../tests/collisions/detection/detection_behaviour.rs"]
mod behaviours;

mod contact;
mod detection;
mod contact_set;

pub mod gjkepa;

pub use self::gjkepa::GJKEPADetection;
pub use self::contact::Contact;
pub use self::contact_set::ContactSet;
pub use self::detection::Detection;
