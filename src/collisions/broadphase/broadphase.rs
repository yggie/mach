use core::{ Database };
use collisions::Contact;

#[cfg(test)]
#[path="../../../tests/behaviour/broadphase_behaviour.rs"]
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

pub mod bruteforce;
