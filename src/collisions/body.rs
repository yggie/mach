use ID;
use maths::{Transform, Vec3D};
use collisions::{BodyData, BodyDef, CollisionObject, CollisionData, CollisionGroup, Narrowphase};
use collisions::shapes::convex_shapes::ConvexShape;
use collisions::narrowphase::{NarrowphaseRef, NarrowphaseRefMut};

#[derive(Clone, Debug)]
pub struct Body<E, N> where E: 'static, N: Narrowphase {
    data: BodyData<N>,
    extension_data: E,
}

impl<E, N> CollisionObject for Body<E, N> where E: 'static, N: Narrowphase {
    type Extension = E;
    type Narrowphase = N;

    fn new(id: ID, def: BodyDef, extra: E) -> Body<E, N> {
        Body {
            data: BodyData::new(id, def),
            extension_data: extra,
        }
    }

    #[inline(always)]
    fn id(&self) -> ID {
        self.data.id()
    }

    #[inline(always)]
    fn data(&self) -> &BodyData<N> {
        &self.data
    }

    #[inline(always)]
    fn data_mut(&mut self) -> &mut BodyData<N> {
        &mut self.data
    }

    #[inline(always)]
    fn group(&self) -> CollisionGroup {
        self.data.group()
    }

    #[inline(always)]
    fn collision_data(&self) -> &CollisionData {
        &self.data.collision_data()
    }

    #[inline(always)]
    fn shape(&self) -> &ConvexShape {
        self.data.shape()
    }

    #[inline(always)]
    fn translation(&self) -> &Vec3D {
        self.data.translation()
    }

    #[inline(always)]
    fn transform(&self) -> &Transform {
        self.data.transform()
    }

    #[inline(always)]
    fn extension_data(&self) -> &E {
        &self.extension_data
    }

    #[inline(always)]
    fn extension_data_mut(&mut self) -> &mut E {
        &mut self.extension_data
    }

    #[inline(always)]
    fn narrowphase_ref(&self) -> NarrowphaseRef<N> {
        self.data.narrowphase_ref()
    }

    #[inline(always)]
    fn narrowphase_ref_mut(&mut self) -> NarrowphaseRefMut<N> {
        self.data.narrowphase_ref_mut()
    }

    #[inline]
    fn split_data_mut(&mut self) -> (&mut BodyData<N>, &mut E) {
        (&mut self.data, &mut self.extension_data)
    }
}
