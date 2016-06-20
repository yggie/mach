use collisions::{CollisionObject, Contact, NarrowphaseData};

pub trait Detection<T> where T: NarrowphaseData {
    fn update(&mut self);
    fn compute_contacts(&mut self, object_0: &CollisionObject<T>, object_1: &CollisionObject<T>) -> Option<Contact<T>>;
}
