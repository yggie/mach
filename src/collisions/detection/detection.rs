use utils::Handle;
use collisions::{Contact, CollisionObject};

pub trait Detection<O> where O: CollisionObject {
    fn update(&mut self);
    fn compute_contacts(&mut self, handle_0: &Handle<O>, handle_1: &Handle<O>) -> Option<Contact<O>>;
}
