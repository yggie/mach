use collisions::{BodyHandle, Contact, Narrowphase};

pub trait Detection<D, N> where N: Narrowphase {
    fn update(&mut self);
    fn compute_contacts(&mut self, handle_0: &BodyHandle<D, N>, handle_1: &BodyHandle<D, N>) -> Option<Contact<D, N>>;
}
