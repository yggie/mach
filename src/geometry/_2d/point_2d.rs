#[cfg(test)]
#[path="../../../tests/support/geometry/_2d/arbitrary_point_2d.rs"]
mod arbitrary;

use std::mem;
use std::ops::Deref;

use maths::_2d::Vec2D;

#[derive(Clone, Debug)]
pub struct Point2D(pub Vec2D);

impl Deref for Vec2D {
    type Target = Point2D;

    fn deref(&self) -> &Self::Target {
        unsafe { mem::transmute(self) }
    }
}
