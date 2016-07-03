use Scalar;
use maths::Motion;
use dynamics::MaterialData;
use collisions::{CollisionData, NarrowphaseData};

#[derive(Clone, Debug)]
pub struct RigidBodyData<T> where T: NarrowphaseData {
    pub mass: Scalar,
    pub motion: Motion,
    pub material_data: MaterialData,
    pub collision_data: CollisionData<T>,
}
//
// impl<T> BodyData<T> for RigidBodyData<T> where T: NarrowphaseData {
//     fn downcast<'a>(&'a self) -> BodyDataRef<'a, T> {
//         BodyDataRef::Rigid(self)
//     }
//
//     fn downcast_mut<'a>(&'a mut self) -> BodyDataRefMut<'a, T> {
//         BodyDataRefMut::Rigid(self)
//     }
//
//     fn collision_data(&self) -> &CollisionData<T> {
//         &self.collision_data
//     }
// }
