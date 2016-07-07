use Scalar;
use maths::{UnitVec3D, Vec3D};
use utils::Handle;
use collisions::{CollisionBody, ContactSet};

#[derive(Clone, Debug)]
pub struct Contact<B> where B: CollisionBody {
    set: ContactSet,
    handles: (Handle<B>, Handle<B>),
}

impl<B> Contact<B> where B: CollisionBody {
    pub fn new(set: ContactSet, handle_0: Handle<B>, handle_1: Handle<B>) -> Contact<B> {
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
    pub fn handles(&self) -> &(Handle<B>, Handle<B>) {
        &self.handles
    }

    #[inline]
    pub fn penetration_depth(&self, index: usize) -> Scalar {
        self.set.penetration_depth(index)
    }
}
