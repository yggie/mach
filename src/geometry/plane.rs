use {Scalar, TOLERANCE};
use maths::{CrossProduct, DotProduct, UnitVec3D, Vec3D};
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

    pub fn from_reference(vertices: (Vec3D, Vec3D, Vec3D), reference_point: Vec3D) -> Plane {
        let edge_01 = vertices.1 - vertices.0;
        let edge_12 = vertices.2 - vertices.1;
        let guess = edge_01.cross(edge_12).normalize();

        let reference_offset = reference_point - vertices.0;
        let normal = if guess.dot(reference_offset) > 0.0 {
            -guess
        } else {
            guess
        };

        return Plane::new(vertices.0, normal);
    }

    #[inline]
    pub fn reversed(self) -> Plane {
        Plane {
            normal: -self.normal,
            .. self
        }
    }

    #[inline]
    pub fn offset_for_origin(&self) -> Scalar {
        -self.normal.dot(self.reference_point)
    }

    #[inline]
    pub fn offset_for(&self, point: Vec3D) -> Scalar {
        self.normal.dot(point - self.reference_point)
    }

    pub fn location_of(&self, point: Vec3D) -> PlaneLocation {
        match self.offset_for(point) {
            x if x > TOLERANCE => PlaneLocation::Above(x),
            x if -x > TOLERANCE => PlaneLocation::Below(x),
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
