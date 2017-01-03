use Scalar;
use maths::{CrossProduct, DotProduct, UnitVec3D, Vec3D};

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

    pub fn from_counter_clockwise_points(a: Vec3D, b: Vec3D, c: Vec3D) -> Plane {
        let edge_ab = b - a;
        let edge_ac = c - a;

        let normal = edge_ab.cross(edge_ac).normalize();

        Plane {
            normal: normal,
            reference_point: a,
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
