use collisions::{BodyHandle, Contact, Narrowphase};

pub trait Detection<N, T> where N: Narrowphase {
    fn update(&mut self);
    fn compute_contacts(&mut self, handle_0: &BodyHandle<N, T>, handle_1: &BodyHandle<N, T>) -> Option<Contact<N, T>>;
}
