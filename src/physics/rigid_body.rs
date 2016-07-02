use ID;
use physics::{Body, BodyRef, BodyRefMut, RigidBodyData};
use collisions::{CollisionData, NarrowphaseData};

#[derive(Clone, Debug)]
pub struct RigidBody<T> where T: NarrowphaseData {
    id: ID,
    data: RigidBodyData<T>,
}

impl<T> RigidBody<T> where T: NarrowphaseData {
    pub fn new(id: ID, data: RigidBodyData<T>) -> RigidBody<T> {
        RigidBody {
            id: id,
            data: data,
        }
    }

    #[inline(always)]
    pub fn id(&self) -> ID {
        self.id
    }

    #[inline(always)]
    pub fn data(&self) -> &RigidBodyData<T> {
        &self.data
    }

    #[inline(always)]
    pub fn data_mut(&mut self) -> &mut RigidBodyData<T> {
        &mut self.data
    }

    #[inline(always)]
    pub fn collision_data(&self) -> &CollisionData<T> {
        &self.data.collision_data
    }

    #[inline(always)]
    pub fn collision_data_mut(&mut self) -> &mut CollisionData<T> {
        &mut self.data.collision_data
    }
}

impl<T> Body<T> for RigidBody<T> where T: NarrowphaseData {
    fn id(&self) -> ID {
        RigidBody::id(self)
    }

    fn downcast(&self) -> BodyRef<T> {
        BodyRef::Rigid(self)
    }

    fn downcast_mut(&mut self) -> BodyRefMut<T> {
        BodyRefMut::Rigid(self)
    }

    #[inline(always)]
    fn collision_data(&self) -> &CollisionData<T> {
        RigidBody::collision_data(self)
    }

    #[inline(always)]
    fn collision_data_mut(&mut self) -> &mut CollisionData<T> {
        RigidBody::collision_data_mut(self)
    }
}
