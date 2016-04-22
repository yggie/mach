use Scalar;
use maths::Vec3D;
use entities::BodyHandle;
use detection::ContactSet;

pub struct ContactEvent {
    bodies: (BodyHandle, BodyHandle),
    contact_set: ContactSet,
}

impl ContactEvent {
    pub fn new(bodies: (BodyHandle, BodyHandle), contact_set: ContactSet) -> ContactEvent {
        ContactEvent {
            bodies: bodies,
            contact_set: contact_set,
        }
    }

    #[inline]
    pub fn bodies(&self) -> &(BodyHandle, BodyHandle) {
        &self.bodies
    }

    #[inline]
    pub fn point(&self, index: usize) -> &Vec3D {
        self.contact_set.point(index)
    }

    #[inline]
    pub fn points(&self) -> &Vec<Vec3D> {
        self.contact_set.points()
    }

    #[inline]
    pub fn normal(&self) -> &Vec3D {
        self.contact_set.normal()
    }

    #[inline]
    pub fn penetration_depth(&self, index: usize) -> Scalar {
        self.contact_set.penetration_depth(index)
    }
}
