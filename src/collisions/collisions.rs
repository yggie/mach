//! Contains an implementation of strategies for optimizing spatial queries.

use bodies::Body;

use std::rc::Rc;
use std::vec::Vec;

pub use self::contact::Contact;
pub use self::proximitypair::ProximityPair;
pub use self::resolution::contactgraph::ContactGraph;
pub use self::detection::broadphase::bruteforce::BruteForce;

#[cfg(test)]
#[path="../../tests/collisions/collisions_test.rs"]
pub mod tests;

/// A Space is a data structure for storing and querying space.
pub trait Space {
    /// Adds a new physical entity to the structure.
    fn add(&mut self, &Rc<Body>);
    /// Returns the number of physical entities in the structure.
    fn size(&self) -> uint;
    /// Traverses the structure to look for any contact. Once a contact is
    /// encountered, the callback function is immediately called.
    fn each_contact(&mut self, |Contact|);
}

mod contact;
mod proximitypair;

mod detection {
    pub mod broadphase {
        pub mod bruteforce;
    }
}

mod resolution {
    pub mod contactgraph;
}
