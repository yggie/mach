#[cfg(test)]
#[path="../../../tests/support/geometry/_2d/arbitrary_ray_2d.rs"]
mod arbitrary;

use Scalar;
use maths::DotProduct;
use maths::_2d::{UnitVec2D, Vec2D};
use geometry::_2d::Plane2D;

#[derive(Clone, Debug)]
pub struct Ray2D {
    source: Vec2D,
    direction: UnitVec2D,
}

impl Ray2D {
    pub fn new(source: Vec2D, direction: UnitVec2D) -> Ray2D {
        Ray2D {
            source: source,
            direction: direction,
        }
    }

    #[inline(always)]
    pub fn source(&self) -> &Vec2D {
        &self.source
    }

    #[inline(always)]
    pub fn direction(&self) -> &UnitVec2D {
        &self.direction
    }

    pub fn reversed(self) -> Ray2D {
        Ray2D::new(self.source, self.direction.reversed())
    }

    pub fn project_along_direction(&self, vec: &Vec2D) -> Scalar {
        self.direction.dot(&(vec - &self.source))
    }

    pub fn counter_clockwise_normal(&self) -> UnitVec2D {
        self.direction.rotate_90()
    }

    pub fn counter_clockwise_plane(&self) -> Plane2D {
        Plane2D::new(self.source.clone(), self.counter_clockwise_normal())
    }
}
