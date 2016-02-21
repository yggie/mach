use entities::{BodyHandle, EntityStore};
use narrowphase::Narrowphase;

pub trait Broadphase {
    type EntityStore: EntityStore;

    fn notify_body_created(&mut self, &Self::EntityStore, &BodyHandle);
    fn update<N: Narrowphase>(&mut self, &N);
    fn contact_candidate_pairs_iter<'a>(&'a self, &'a Self::EntityStore) -> Box<Iterator<Item=(BodyHandle, BodyHandle)> + 'a>;
}
