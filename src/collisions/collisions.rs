//! Contains an implementation of strategies for optimizing spatial queries.

use core::{ Database };

pub use self::contact::Contact;
pub use self::proximitypair::ProximityPair;
pub use self::broadphase::bruteforce::BruteForce;

#[cfg(test)]
#[path="../../tests/behaviour/broadphase_behaviour.rs"]
mod behaviours;

/// The `BroadPhase` component enhances the `Database` component by enabling
/// access to bodies via spatial queries.
pub trait BroadPhase {
    /// Rebuilds the spatial indices for the structure.
    fn reindex(&mut self, &Database);
    /// Queries the structure for contacting `Body` instances, calling the
    /// provided callback function for each contact found.
    fn each_contact(&mut self, &Database, |Contact|);
}

mod contact;
mod proximitypair;

mod broadphase {
    pub mod bruteforce;
}
