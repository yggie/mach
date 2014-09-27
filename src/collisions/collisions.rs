//! Contains an implementation of strategies for optimizing spatial queries.

use bodies::Body;

use std::rc::Rc;
use std::vec::Vec;

pub use self::resolution::contactgraph::ContactGraph;
pub use self::detection::broadphase::bruteforce::BruteForce;

#[cfg(test)]
#[path="../../tests/collisions/collisions_test.rs"]
pub mod tests;

/// Defines the trait for all spatial partitioning strategies to implement.
pub trait BroadPhase {
    /// Adds a new body to the structure.
    fn add(&mut self, &Rc<Body>);
    /// Returns the number of bodies in the structure.
    fn count(&self) -> uint;
    /// Returns all the partitions in the structure.
    fn partitions(&self) -> &Vec<Vec<Rc<Body>>>;
}

mod detection {
    pub mod broadphase {
        pub mod bruteforce;
    }
}

mod resolution {
    pub mod contactgraph;
}
