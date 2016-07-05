use collisions::{BodyData, CollisionData};

pub trait Narrowphase: 'static + Clone {
    fn new(data: &CollisionData) -> Self;
    fn test(body_0: &BodyData<Self>, body_1: &BodyData<Self>) -> bool;
    fn update(body: &mut BodyData<Self>);
}
