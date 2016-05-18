use collisions::CollisionObject;

pub trait CollisionObjectLifecycleEventListener<D> {
    fn on_object_created(&mut self, object: &CollisionObject<D>);
}
