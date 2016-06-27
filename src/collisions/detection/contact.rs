use maths::{UnitVec3D, Vec3D};
use detection::ContactSet;
use collisions::{CollisionObject, NarrowphaseData};

pub struct Contact<T> where T: NarrowphaseData {
    set: ContactSet,
    objects: (CollisionObject<T>, CollisionObject<T>),
}

impl<T> Contact<T> where T: NarrowphaseData {
    pub fn new(set: ContactSet, object_0: CollisionObject<T>, object_1: CollisionObject<T>) -> Contact<T> {
        Contact {
            set: set,
            objects: (object_0, object_1),
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
}
