use Scalar;
use maths::{UnitVec3D, Vec3D};
use utils::Handle;
use collisions::{ContactSet, CollisionObject};

#[derive(Clone, Debug)]
pub struct Contact<O> where O: CollisionObject {
    set: ContactSet,
    handles: (Handle<O>, Handle<O>),
}

impl<O> Contact<O> where O: CollisionObject {
    pub fn new(set: ContactSet, handle_0: Handle<O>, handle_1: Handle<O>) -> Contact<O> {
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
    pub fn handles(&self) -> &(Handle<O>, Handle<O>) {
        &self.handles
    }

    #[inline]
    pub fn penetration_depth(&self, index: usize) -> Scalar {
        self.set.penetration_depth(index)
    }
}
