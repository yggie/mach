use core::{ Body, Database };
use collisions::{ BroadPhase, Contact, ProximityPair };

#[cfg(test)]
#[path="../../../tests/unit/collisions/detection/broadphase/bruteforce_test.rs"]
mod tests;

/// Represents a brute force approach for partitioning space. The entire
/// world is considered a single partition.
pub struct BruteForce {
    pairs: Vec<ProximityPair>,
}

impl BruteForce {
    /// Creates a new empty `BruteForce` instance.
    pub fn new() -> BruteForce {
        BruteForce{ pairs: Vec::new() }
    }
}

impl BroadPhase for BruteForce {
    fn reindex(&mut self, database: &Database) {
        self.pairs = Vec::new();
        database.each_body_pair(|body_0: &Body, body_1: &Body| {
            self.pairs.push(ProximityPair::new(body_0, body_1));
        });
    }

    fn each_contact(&mut self, database: &Database, callback: |Contact|) {
        for pair in self.pairs.iter_mut() {
            match (database.find(pair.body_ids[0]), database.find(pair.body_ids[1])) {
                (Some(body_0), Some(body_1)) => {
                    match pair.compute_contact(body_0, body_1) {
                        None => (),
                        Some(contact) => callback(contact),
                    }
                },

                // TODO handle missing bodies
                _ => continue,
            }
        }
    }
}
