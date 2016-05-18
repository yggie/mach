use ID;
use collisions::CollisionDataHandle;

#[derive(Clone, Debug)]
pub struct CollisionObject<T> {
    pub id: ID,
    // TODO should we support more than one group?
    pub is_background: bool,
    pub data: CollisionDataHandle<T>,
}
