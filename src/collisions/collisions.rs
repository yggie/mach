//! Contains an implementation of strategies for optimizing spatial queries.

pub use self::contact::Contact;
pub use self::broadphase::BroadPhase;
pub use self::proximitypair::ProximityPair;
pub use self::broadphase::bruteforce::BruteForce;

#[path="broadphase/broadphase.rs"]
mod broadphase;
mod contact;
mod proximitypair;
