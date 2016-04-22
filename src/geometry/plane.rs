use {Scalar, TOLERANCE};
use maths::{DotProduct, UnitVec3D, Vec3D};
use geometry::PlaneLocation;

#[derive(Clone)]
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
    pub fn projection_of_origin_along_normal(&self) -> Scalar {
        -self.normal.dot(self.reference_point)
    }

    #[inline]
    pub fn project_along_normal(&self, point: Vec3D) -> Scalar {
        self.normal.dot(point - self.reference_point)
    }

    pub fn projection_of(&self, point: Vec3D) -> PlaneLocation {
        match self.project_along_normal(point) {
            x if x > TOLERANCE => PlaneLocation::Above(x),
            x if x < -TOLERANCE => PlaneLocation::Below(x),
            x => PlaneLocation::OnPlane(x),
        }
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
