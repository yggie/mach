use {Scalar, TOLERANCE};
use maths::DotProduct;
use maths::_2d::{UnitVec2D, Vec2D};
use geometry::PlaneLocation;

#[derive(Clone, Debug)]
pub struct Plane2D {
    normal: UnitVec2D,
    reference_point: Vec2D,
}

impl Plane2D {
    pub fn new(reference_point: Vec2D, normal: UnitVec2D) -> Plane2D {
        Plane2D {
            normal: normal,
            reference_point: reference_point,
        }
    }

    pub fn from_points(a: &Vec2D, b: &Vec2D) -> Plane2D {
        Plane2D {
            normal: (b - a).rotate_90().normalize(),
            reference_point: a.clone(),
        }
    }

    #[inline]
    pub fn normal(&self) -> &UnitVec2D {
        &self.normal
    }

    #[inline]
    pub fn project_along_normal(&self, vec2: &Vec2D) -> Scalar {
        self.normal.dot(&(vec2 - &self.reference_point))
    }

    pub fn projection_of(&self, vec2: &Vec2D) -> PlaneLocation {
        match self.project_along_normal(vec2) {
            x if x > TOLERANCE => PlaneLocation::Above(x),
            x if -x > TOLERANCE => PlaneLocation::Below(x),
            x => PlaneLocation::OnPlane(x),
        }
    }
}
