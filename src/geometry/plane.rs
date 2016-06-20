use Scalar;
use maths::{DotProduct, UnitVec3D, Vec3D};

#[derive(Clone, Debug)]
pub struct Plane {
    normal: UnitVec3D,
    reference_point: Vec3D,
}

impl Plane {
    pub fn new(point: Vec3D, normal: UnitVec3D) -> Plane {
        Plane {
            normal: normal,
            reference_point: point,
        }
    }

    #[inline]
    pub fn reversed(self) -> Plane {
        Plane {
            normal: -self.normal,
            .. self
        }
    }

    #[inline]
    pub fn normal_projection_of_origin(&self) -> Scalar {
        -self.normal.dot(self.reference_point)
    }

    #[inline]
    pub fn normal_projection_of(&self, point: Vec3D) -> Scalar {
        self.normal.dot(point - self.reference_point)
    }

    #[inline]
    pub fn normal(&self) -> UnitVec3D {
        self.normal
    }

    #[inline]
    pub fn reference(&self) -> Vec3D {
        self.reference_point
    }
}
