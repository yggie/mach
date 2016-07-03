use collisions::{Body, CollisionData};

pub trait Narrowphase: 'static + Clone {
    fn new(data: &CollisionData) -> Self;
    fn test<D>(body_0: &Body<D, Self>, body_1: &Body<D, Self>) -> bool;
    fn update<D>(body: &mut Body<D, Self>);
}
