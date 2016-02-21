use ID;
use entities::{Body, EntityStore};

pub trait Narrowphase {
    type EntityStore: EntityStore;

    fn notify_body_created(&mut self, &Self::EntityStore, &Body);
    fn update(&mut self, &Self::EntityStore);
    // possibly could be preloaded with positional data
    fn test(&self, ID, ID) -> bool;
}
