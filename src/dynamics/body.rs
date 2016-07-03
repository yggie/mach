use ID;
use dynamics::{BodyRef, BodyRefMut};
use collisions::{CollisionData, NarrowphaseData};

pub trait Body<T> where T: NarrowphaseData {
    fn id(&self) -> ID;
    fn downcast(&self) -> BodyRef<T>;
    fn downcast_mut(&mut self) -> BodyRefMut<T>;
    fn collision_data(&self) -> &CollisionData<T>;
    fn collision_data_mut(&mut self) -> &mut CollisionData<T>;
}

// pub struct RigidBodyData {
//     mass: Scalar,
//     motion: Motion,
//     material_data: MaterialData,
// }
//
// pub struct FixedBodyData {
//     material_data: MaterialData,
// }
//
// pub enum DynamicData {
//     Rigid(Box<RigidBodyData>),
//     Fixed(Box<FixedBodyData>),
// }
//
// pub struct NewBody<D, T> where T: NarrowphaseData {
//     id: ID,
//     extra_data: D,
//     group_flag: u32,
//     collision_data: CollisionData<T>,
// }
