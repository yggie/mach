//! Contains an implementation of strategies for resolving collisions.

use bodies::Body;

use std::rc::Rc;
use std::vec::Vec;

pub use self::broadphase::BruteForce;

#[cfg(test)]
#[path="../../tests/collisions/collisions_test.rs"]
pub mod tests;

/// Defines the trait for all spatial partitioning strategies to implement.
trait BroadPhase {
    /// Adds a new body to the structure.
    fn add(&mut self, &Rc<Body>);
    /// Returns the number of bodies in the structure.
    fn count(&self) -> uint;
    /// Returns all the partitions in the structure.
    fn partitions(&self) -> &Vec<Vec<Rc<Body>>>;
}

mod broadphase {
    pub use self::bruteforce::BruteForce;

    mod bruteforce;
}
