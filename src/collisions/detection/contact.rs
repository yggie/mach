use detection::ContactSet;
use collisions::{CollisionObject, NarrowphaseData};

pub struct Contact<T> where T: NarrowphaseData {
    set: ContactSet,
    objects: (CollisionObject<T>, CollisionObject<T>),
}

impl<T> Contact<T> where T: NarrowphaseData {
    pub fn new(set: ContactSet, object_0: CollisionObject<T>, object_1: CollisionObject<T>) -> Contact<T> {
        Contact {
            set: set,
            objects: (object_0, object_1),
        }
    }
}
