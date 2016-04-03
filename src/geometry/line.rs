#[cfg(test)]
#[path="../../tests/geometry/line_test.rs"]
mod tests;

use Scalar;
use maths::Vect;

pub struct Line {
    datum: Vect,
    direction: Vect,
}

impl Line {
    #[inline]
    pub fn new(datum: Vect, direction: Vect) -> Line {
        Line {
            datum: datum,
            direction: direction.normalize(),
        }
    }

    #[inline]
    pub fn from_points(start: Vect, end: Vect) -> Line {
        Line::new(start, end - start)
    }

    #[inline]
    pub fn direction(&self) -> &Vect {
        &self.direction
    }

    #[inline]
    pub fn datum(&self) -> &Vect {
        &self.datum
    }

    pub fn point_with_offset(&self, offset: Scalar) -> Vect {
        self.datum + self.direction * offset
    }

    pub fn closest_point_to_line(&self, other: &Line) -> Vect {
        let w = self.datum() - other.datum();
        let a = Vect::dot(self.direction(), self.direction().clone());
        let b = Vect::dot(self.direction(), other.direction().clone());
        let c = Vect::dot(other.direction(), other.direction().clone());
        let d = Vect::dot(self.direction(), w);
        let e = Vect::dot(other.direction(), w);

        let denominator = a*c - b*b;
        let self_offset = (b*e - c*d) / denominator;
        let other_offset = (a*e - b*d) / denominator;

        let closest_point_on_self = self.point_with_offset(self_offset);
        let closets_point_on_other = other.point_with_offset(other_offset);

        return (closest_point_on_self + closets_point_on_other) / 2.0;
    }
}
