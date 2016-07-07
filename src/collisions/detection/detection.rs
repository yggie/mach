use utils::Handle;
use collisions::{CollisionBody, Contact};

pub trait Detection<B> where B: CollisionBody {
    fn update(&mut self);
    fn compute_contacts(&mut self, handle_0: &Handle<B>, handle_1: &Handle<B>) -> Option<Contact<B>>;
}
