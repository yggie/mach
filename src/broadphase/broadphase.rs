use ID;
use entities::{Body, EntityStore};

use temp::Narrowphase;

pub trait Broadphase {
    type EntityStore: EntityStore;

    fn notify_body_created(&mut self, &Self::EntityStore, &Body);
    fn update<N: Narrowphase>(&mut self, &Self::EntityStore, &N);
    fn contact_candidate_pairs_iter<'a>(&'a self, &'a Self::EntityStore) -> Box<Iterator<Item=(ID, ID)> + 'a>;
}
