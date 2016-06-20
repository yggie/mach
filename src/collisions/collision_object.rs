use ID;
use collisions::{CollisionDataHandle, NarrowphaseData};

#[derive(Clone, Debug)]
pub struct CollisionObject<T> where T: NarrowphaseData {
    pub id: ID,
    // TODO should we support more than one group?
    pub is_background: bool,
    pub data: CollisionDataHandle<T>,
}
