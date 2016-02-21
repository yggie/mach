use entities::BodyHandle;
use detection::ContactEvent;

pub trait Detection {
    fn update(&mut self);
    fn compute_contacts(&mut self, &BodyHandle, &BodyHandle) -> Option<ContactEvent>;
}
