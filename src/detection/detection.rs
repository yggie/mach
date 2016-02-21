use ID;
use entities::EntityStore;
use detection::ContactEvent;

pub trait Detection {
    type EntityStore: EntityStore;

    fn update(&mut self);
    fn compute_contacts(&mut self, &Self::EntityStore, ID, ID) -> Option<ContactEvent>;
}
