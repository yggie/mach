#[cfg(test)]
#[path="../../../tests/collisions/geometry/direction_test.rs"]
mod direction_test;

use std::ops::Neg;

use maths::{Transform, UnitQuat, UnitVec3D, Vec3D};

#[derive(Clone, Copy, Debug)]
pub struct Direction(Vec3D);

impl Direction {
    pub fn rotate_by(&self, rotation: UnitQuat) -> Direction {
        Direction::from(rotation.rotate(Vec3D::from(*self)))
    }

    pub fn transform_with(&self, transform: &Transform) -> Direction {
        self.rotate_by(transform.rotation())
    }

    pub fn transform_with_inverse_of(&self, transform: &Transform) -> Direction {
        self.rotate_by(transform.rotation().inverse())
    }
}

impl From<Vec3D> for Direction {
    fn from(vec: Vec3D) -> Direction {
        Direction(vec)
    }
}

impl From<Direction> for Vec3D {
    fn from(point: Direction) -> Vec3D {
        point.0
    }
}

impl From<UnitVec3D> for Direction {
    fn from(unit_vec: UnitVec3D) -> Direction {
        Direction::from(Vec3D::from(unit_vec))
    }
}

impl From<Direction> for UnitVec3D {
    fn from(direction: Direction) -> UnitVec3D {
        UnitVec3D::from(Vec3D::from(direction))
    }
}

impl Neg for Direction {
    type Output = Direction;

    #[inline]
    fn neg(self) -> Direction {
        Direction::from(-Vec3D::from(self))
    }
}
