#[cfg(test)]
#[path="../../../../tests/support/collisions/shapes/_2d/arbitrary_point_2d.rs"]
mod arbitrary;

use std::mem;
use std::ops::Deref;

use maths::_2d::Vec2D;

use collisions::shapes::Shape;

#[derive(Clone, Debug)]
pub struct Point2D(pub Vec2D);

impl Shape for Point2D {}

impl From<Point2D> for Vec2D {
    fn from(point: Point2D) -> Vec2D {
        point.0
    }
}

impl Deref for Vec2D {
    type Target = Point2D;

    fn deref(&self) -> &Self::Target {
        unsafe { mem::transmute(self) }
    }
}
