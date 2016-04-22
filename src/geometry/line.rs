#[cfg(test)]
#[path="../../tests/geometry/line_test.rs"]
mod tests;

use Scalar;
use maths::Vec3D;

pub struct Line {
    datum: Vec3D,
    direction: Vec3D,
}

impl Line {
    #[inline]
    pub fn new(datum: Vec3D, direction: Vec3D) -> Line {
        Line {
            datum: datum,
            direction: direction.normalize(),
        }
    }

    #[inline]
    pub fn from_points(start: Vec3D, end: Vec3D) -> Line {
        Line::new(start, end - start)
    }

    #[inline]
    pub fn direction(&self) -> &Vec3D {
        &self.direction
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
        let a = Vec3D::dot(self.direction(), self.direction().clone());
        let b = Vec3D::dot(self.direction(), other.direction().clone());
        let c = Vec3D::dot(other.direction(), other.direction().clone());
        let d = Vec3D::dot(self.direction(), w);
        let e = Vec3D::dot(other.direction(), w);

        let denominator = a*c - b*b;
        let self_offset = (b*e - c*d) / denominator;
        let other_offset = (a*e - b*d) / denominator;

        let closest_point_on_self = self.point_with_offset(self_offset);
        let closets_point_on_other = other.point_with_offset(other_offset);

        return (closest_point_on_self + closets_point_on_other) / 2.0;
    }
}
