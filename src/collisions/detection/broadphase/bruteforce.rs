use bodies::Body;
use collisions::{ Space, Contact };

use std::rc::Rc;

#[cfg(test)]
#[path="../../../../tests/collisions/detection/broadphase/bruteforce_test.rs"]
mod tests;

/// Represents a brute force approach for partitioning space. The entire
/// world is considered a single partition.
pub struct BruteForce {
    bodies: Vec<Rc<Body>>,
    count: uint,
}

impl BruteForce {

    /// Instantiates a new BruteForce strategy for spatial partitioning.
    pub fn new() -> BruteForce {
        BruteForce{ bodies: Vec::new(), count: 0 }
    }
}

impl Space for BruteForce {

    /// Adds the body to the structure.
    fn add(&mut self, body: &Rc<Body>) {
        self.count += 1;
        self.bodies.push(body.clone());
    }

    /// Returns the number of bodies contained in the structure.
    fn size(&self) -> uint {
        self.count
    }

    /// Traverses the structure to look for any contact. Once a contact is
    /// encountered, the callback function is immediately called.
    fn each_contact(&mut self, callback: |Contact|) {
        let total = self.bodies.len();
        for i in range(0u, total) {
            let a = &self.bodies[i];
            for j in range(i + 1u, total) {
                let b = &self.bodies[j];

                callback(Contact::new(a.clone(), b.clone()));
            }
        }
    }
}
