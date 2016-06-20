use collisions::{CollisionObject, NarrowphaseData};

pub trait CollisionObjectLifecycleEventListener<T> where T: NarrowphaseData {
    fn on_object_created(&mut self, object: &CollisionObject<T>);
}
