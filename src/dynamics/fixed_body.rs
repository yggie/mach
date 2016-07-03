use ID;
use dynamics::{Body, BodyRef, BodyRefMut, FixedBodyData};
use collisions::{CollisionData, NarrowphaseData};

#[derive(Clone, Debug)]
pub struct FixedBody<T> where T: NarrowphaseData {
    id: ID,
    data: FixedBodyData<T>,
}

impl<T> FixedBody<T> where T: NarrowphaseData {
    pub fn new(id: ID, data: FixedBodyData<T>) -> FixedBody<T> {
        FixedBody {
            id: id,
            data: data,
        }
    }

    #[inline(always)]
    pub fn id(&self) -> ID {
        self.id
    }

    #[inline(always)]
    pub fn data(&self) -> &FixedBodyData<T> {
        &self.data
    }

    #[inline(always)]
    pub fn data_mut(&mut self) -> &mut FixedBodyData<T> {
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

impl<T> Body<T> for FixedBody<T> where T: NarrowphaseData {
    fn id(&self) -> ID {
        FixedBody::id(self)
    }

    fn downcast(&self) -> BodyRef<T> {
        BodyRef::Fixed(self)
    }

    fn downcast_mut(&mut self) -> BodyRefMut<T> {
        BodyRefMut::Fixed(self)
    }

    #[inline(always)]
    fn collision_data(&self) -> &CollisionData<T> {
        FixedBody::collision_data(self)
    }

    #[inline(always)]
    fn collision_data_mut(&mut self) -> &mut CollisionData<T> {
        FixedBody::collision_data_mut(self)
    }
}
