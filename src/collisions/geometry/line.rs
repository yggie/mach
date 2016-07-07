#[cfg(test)]
#[path="../../../tests/collisions/geometry/line_test.rs"]
mod tests;

use Scalar;
use maths::{DotProduct, UnitVec3D, Vec3D};

pub struct Line {
    datum: Vec3D,
    direction: UnitVec3D,
}

impl Line {
    #[inline]
    pub fn new(datum: Vec3D, direction: UnitVec3D) -> Line {
        Line {
            datum: datum,
            direction: direction,
        }
    }

    #[inline]
    pub fn from_points(start: Vec3D, end: Vec3D) -> Line {
        Line::new(start, (end - start).normalize())
    }

    #[inline]
    pub fn direction(&self) -> UnitVec3D {
        self.direction
    }

    #[inline]
    pub fn datum(&self) -> &Vec3D {
        &self.datum
    }

    pub fn point_with_offset(&self, offset: Scalar) -> Vec3D {
        self.datum + self.direction * offset
    }

    pub fn closest_point_to_line(&self, other: &Line) -> Vec3D {
        let w = self.datum() - other.datum();
        let a = self.direction().dot(self.direction());
        let b = self.direction().dot(other.direction());
        let c = other.direction().dot(other.direction());
        let d = self.direction().dot(w);
        let e = other.direction().dot(w);

        let denominator = a*c - b*b;
        let self_offset = (b*e - c*d) / denominator;
        let other_offset = (a*e - b*d) / denominator;

        let closest_point_on_self = self.point_with_offset(self_offset);
        let closets_point_on_other = other.point_with_offset(other_offset);

        return (closest_point_on_self + closets_point_on_other) / 2.0;
    }
}
