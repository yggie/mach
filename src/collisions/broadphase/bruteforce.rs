use bodies::Body;
use collisions::BroadPhase;

use std::rc::Rc;

#[cfg(test)]
#[path="../../../tests/collisions/broadphase/bruteforce_test.rs"]
mod tests;

/// Represents a brute force approach for partitioning space. The entire
/// world is considered a single partition.
pub struct BruteForce {
    partitions: Vec<Vec<Rc<Body>>>,
    count: uint,
}

impl BruteForce {

    /// Instantiates a new BruteForce strategy for spatial partitioning.
    pub fn new() -> BruteForce {
        let mut p = Vec::new();
        p.push(Vec::new());
        BruteForce{ partitions: p, count: 0 }
    }
}

impl BroadPhase for BruteForce {

    /// Adds the body to the structure.
    fn add(&mut self, body: &Rc<Body>) {
        self.count += 1;
        self.partitions.get_mut(0).push(body.clone());
    }

    /// Returns the number of bodies contained in the structure.
    fn count(&self) -> uint {
        self.count
    }

    /// Returns all the spatial partitions in the structure.
    fn partitions(&self) -> &Vec<Vec<Rc<Body>>> {
        &self.partitions
    }
}
