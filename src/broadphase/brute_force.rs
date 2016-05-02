#[cfg(test)]
#[path="../../tests/broadphase/brute_force_test.rs"]
mod tests;

use std::marker::PhantomData;

use ID;
use entities::{BodyHandle, BodyRef, EntityStore};
use broadphase::Broadphase;
use narrowphase::Narrowphase;

pub struct BruteForce<ES: EntityStore> {
    pairs: Vec<(ID, ID)>,
    _phantom: PhantomData<ES>,
}

impl<ES: EntityStore> BruteForce<ES> {
    pub fn new() -> BruteForce<ES> {
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

    fn notify_body_created(&mut self, store: &Self::EntityStore, handle: &BodyHandle) {
        let body = handle.borrow();

        match body.downcast() {
            BodyRef::Rigid(_rigid_body) => {
                for other_body in store.bodies_iter() {
                    self.attempt_store_pair(body.id(), other_body.id());
                }
            },

            BodyRef::Static(_static_body) => {
                for other_body in store.bodies_iter() {
                    if let BodyRef::Static(_static_body) = other_body.downcast() {
                        continue;
                    }

                    self.attempt_store_pair(body.id(), other_body.id());
                }
            },
        }
    }

    fn update<N: Narrowphase>(&mut self, _narrowphase: &N) {
        // do nothing
    }

    fn contact_candidate_pairs_iter<'a>(&'a self, store: &'a Self::EntityStore) -> Box<Iterator<Item=(BodyHandle, BodyHandle)> + 'a> {
        let iterator = self.pairs.iter()
            .map(move |pair| {
                let handle_0 = store.find_body_handle(pair.0).unwrap();
                let handle_1 = store.find_body_handle(pair.1).unwrap();

                (handle_0.clone(), handle_1.clone())
            });

        Box::new(iterator)
    }
}
