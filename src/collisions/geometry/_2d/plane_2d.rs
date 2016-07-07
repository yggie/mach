#[cfg(test)]
#[path="../../../../tests/collisions/geometry/_2d/plane_2d_test.rs"]
mod tests;

use Scalar;
use maths::DotProduct;
use maths::_2d::{UnitVec2D, Vec2D};

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

    #[inline(always)]
    pub fn normal(&self) -> &UnitVec2D {
        &self.normal
    }

    #[inline]
    pub fn normal_projection_of(&self, vec2: &Vec2D) -> Scalar {
        self.normal.dot(&(vec2 - &self.reference_point))
    }
}
