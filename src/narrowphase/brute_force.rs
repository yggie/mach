#[cfg(test)]
#[path="../../tests/narrowphase/brute_force_test.rs"]
mod tests;

use std::marker::PhantomData;

use ID;
use entities::{Body, EntityStore};
use narrowphase::Narrowphase;

pub struct BruteForce<ES: EntityStore> {
    _phantom: PhantomData<ES>
}

impl<ES: EntityStore> BruteForce<ES> {
    pub fn new() -> BruteForce<ES> {
        BruteForce {
            _phantom: PhantomData,
        }
    }
}

impl<ES: EntityStore> Narrowphase for BruteForce<ES> {
    type EntityStore = ES;

    fn notify_body_created(&mut self, _store: &Self::EntityStore, _body: &Body) {
        // do nothing
    }

    fn update(&mut self, _store: &Self::EntityStore) {
        // do nothing
    }

    fn test(&self, _id_0: ID, _id_1: ID) -> bool {
        true
    }
}
