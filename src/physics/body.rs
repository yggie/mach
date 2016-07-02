use ID;
use physics::{BodyRef, BodyRefMut};
use collisions::{CollisionData, NarrowphaseData};

pub trait Body<T> where T: NarrowphaseData {
    fn id(&self) -> ID;
    fn downcast(&self) -> BodyRef<T>;
    fn downcast_mut(&mut self) -> BodyRefMut<T>;
    fn collision_data(&self) -> &CollisionData<T>;
    fn collision_data_mut(&mut self) -> &mut CollisionData<T>;
}
