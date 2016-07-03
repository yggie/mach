use maths::{UnitVec3D, Vec3D};
use detection::ContactSet;
use collisions::{BodyHandle, Narrowphase};

#[derive(Clone, Debug)]
pub struct Contact<D, N> where N: Narrowphase {
    set: ContactSet,
    handles: (BodyHandle<D, N>, BodyHandle<D, N>),
}

impl<D, N> Contact<D, N> where N: Narrowphase {
    pub fn new(set: ContactSet, handle_0: BodyHandle<D, N>, handle_1: BodyHandle<D, N>) -> Contact<D, N> {
        Contact {
            set: set,
            handles: (handle_0, handle_1),
        }
    }

    #[inline]
    pub fn point(&self, index: usize) -> Vec3D {
        self.set.point(index)
    }

    #[inline]
    pub fn points(&self) -> &Vec<Vec3D> {
        self.set.points()
    }

    #[inline(always)]
    pub fn normal(&self) -> UnitVec3D {
        self.set.normal()
    }

    #[inline(always)]
    pub fn handles(&self) -> &(BodyHandle<D, N>, BodyHandle<D, N>) {
        &self.handles
    }
}
