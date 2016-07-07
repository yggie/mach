use collisions::CollisionData;
use collisions::narrowphase::{NarrowphaseRef, NarrowphaseRefMut};

pub trait Narrowphase: 'static + Clone {
    fn new(body: &CollisionData) -> Self;
    fn test(ref_0: NarrowphaseRef<Self>, ref_1: NarrowphaseRef<Self>) -> bool;
    fn update(data: NarrowphaseRefMut<Self>);
}
