//! Contains an implementation of strategies for resolving collisions.

use bodies::Body;

use std::rc::Rc;
use std::vec::Vec;

pub use self::broadphase::BruteForce;

#[cfg(test)]
#[path="../../tests/collisions/collisions_test.rs"]
pub mod tests;

trait BroadPhase {
    fn add(&mut self, &Rc<Body>);
    fn count(&self) -> uint;
    fn partitions(&self) -> &Vec<Vec<Rc<Body>>>;
}

mod broadphase {
    pub use self::bruteforce::BruteForce;

    mod bruteforce;
}
