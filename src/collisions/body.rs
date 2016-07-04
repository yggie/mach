use ID;
use collisions::{BodyData, BodyDef, CollisionData, CollisionGroup, Narrowphase};

#[derive(Clone, Debug)]
pub struct Body<N, T> where N: Narrowphase {
    data: BodyData<N>,
    extra_data: T,
}

impl<N, T> Body<N, T> where N: Narrowphase {
    pub fn new(id: ID, def: BodyDef<T>) -> Body<N, T> {
        Body {
            data: BodyData::new(id, def.group, def.shape, def.transform),
            extra_data: def.extra_data,
        }
    }

    #[inline(always)]
    pub fn id(&self) -> ID {
        self.data.id()
    }

    #[inline(always)]
    pub fn data(&self) -> &BodyData<N> {
        &self.data
    }

    #[inline(always)]
    pub fn group(&self) -> CollisionGroup {
        self.data.group()
    }

    #[inline(always)]
    pub fn collision_data(&self) -> &CollisionData {
        &self.data.collision_data()
    }

    #[inline(always)]
    pub fn extra_data(&self) -> &T {
        &self.extra_data
    }

    #[inline(always)]
    pub fn extra_data_mut(&mut self) -> &mut T {
        &mut self.extra_data
    }

    #[inline]
    pub fn split_data_mut(&mut self) -> (&mut BodyData<N>, &mut T) {
        (&mut self.data, &mut self.extra_data)
    }
}
