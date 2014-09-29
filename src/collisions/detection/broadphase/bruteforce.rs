use bodies::Body;
use collisions::{ Space, Contact, ProximityPair };

use std::rc::Rc;

#[cfg(test)]
#[path="../../../../tests/collisions/detection/broadphase/bruteforce_test.rs"]
mod tests;

/// Represents a brute force approach for partitioning space. The entire
/// world is considered a single partition.
pub struct BruteForce<'a> {
    bodies: Vec<Rc<Body<'a>>>,
    pairs: Vec<ProximityPair<'a>>,
    count: uint,
}

impl<'a> BruteForce<'a> {

    /// Instantiates a new BruteForce strategy for spatial partitioning.
    pub fn new() -> BruteForce<'a> {
        BruteForce{ bodies: Vec::new(), pairs: Vec::new(), count: 0 }
    }
}

impl<'a> Space<'a> for BruteForce<'a> {

    /// Adds the body to the structure.
    fn add(&mut self, body: &Rc<Body>) {
        self.count += 1;
        for b in self.bodies.iter() {
            self.pairs.push(ProximityPair::new(b.clone(), body.clone()));
        }
        self.bodies.push(body.clone());
    }

    /// Returns the number of bodies contained in the structure.
    fn size(&self) -> uint {
        self.count
    }

    /// Traverses the structure to look for any contact. Once a contact is
    /// encountered, the callback function is immediately called.
    fn each_contact(&mut self, callback: |Contact<'a>|) {
        for pair in self.pairs.iter() {
            pair.if_contact(|contact| callback(contact));
        }
    }
}
