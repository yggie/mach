use Scalar;
use maths::{UnitVec3D, Vec3D};
use detection::ContactSet;
use collisions::{BodyHandle, Narrowphase};

#[derive(Clone, Debug)]
pub struct Contact<N, T> where N: Narrowphase {
    set: ContactSet,
    handles: (BodyHandle<N, T>, BodyHandle<N, T>),
}

impl<N, T> Contact<N, T> where N: Narrowphase {
    pub fn new(set: ContactSet, handle_0: BodyHandle<N, T>, handle_1: BodyHandle<N, T>) -> Contact<N, T> {
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
    pub fn handles(&self) -> &(BodyHandle<N, T>, BodyHandle<N, T>) {
        &self.handles
    }

    #[inline]
    pub fn penetration_depth(&self, index: usize) -> Scalar {
        self.set.penetration_depth(index)
    }
}
