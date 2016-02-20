#[cfg(test)]
#[path="../../tests/broadphase/brute_force_test.rs"]
mod tests;

use std::marker::PhantomData;

use ID;
use entities::{Body, BodyType, EntityStore};
use broadphase::Broadphase;

use temp::Narrowphase;

pub struct BruteForce<ES: EntityStore> {
    pairs: Vec<(ID, ID)>,
    _phantom: PhantomData<ES>,
}

impl<ES: EntityStore> BruteForce<ES> {
    fn new() -> BruteForce<ES> {
        BruteForce {
            pairs: Vec::new(),
            _phantom: PhantomData,
        }
    }

    fn attempt_store_pair(&mut self, id: ID, other_id: ID) {
        match (id, other_id) {
            (a, b) if a == b => (),

            (a, b) if a > b => {
                self.pairs.push((b, a));
            },

            (a, b) => {
                self.pairs.push((a, b));
            },
        }
    }
}

impl<ES: EntityStore> Broadphase for BruteForce<ES> {
    type EntityStore = ES;

    fn notify_body_created(&mut self, store: &Self::EntityStore, body: &Body) {
        match body.downcast() {
            BodyType::Rigid(_rigid_body) => {
                for other_body in store.bodies_iter() {
                    self.attempt_store_pair(body.id(), other_body.id());
                }
            },

            BodyType::Static(_static_body) => {
                for other_body in store.bodies_iter() {
                    if let BodyType::Static(_static_body) = other_body.downcast() {
                        continue;
                    }

                    self.attempt_store_pair(body.id(), other_body.id());
                }
            },
        }
    }

    fn update<N: Narrowphase>(&mut self, _store: &Self::EntityStore, _narrowphase: &N) {
        // do nothing
    }

    fn contact_candidate_pairs_iter<'a>(&'a self, _store: &'a Self::EntityStore) -> Box<Iterator<Item=(ID, ID)> + 'a> {
        Box::new(self.pairs.iter().map(|pair| pair.clone()))
    }
}
