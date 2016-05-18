use detection::ContactEvent;
use collisions::{CollisionObject, CollisionObjectLifecycleEventListener};

pub trait Detection<D>: CollisionObjectLifecycleEventListener<D> {
    fn update(&mut self);
    fn compute_contacts(&self, object_0: &CollisionObject<D>, object_1: &CollisionObject<D>) -> Option<ContactEvent>;
}
