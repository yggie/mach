use maths::Approximations;
use collisions::geometry::Intersection;
use collisions::geometry::_2d::{Point2D, Polygon};

impl Intersection<Point2D> for Polygon {
    type Output = Point2D;

    fn intersection(&self, point: &Point2D) -> Option<Self::Output> {
        for plane in self.separating_planes_iter() {
            if plane.normal_projection_of(&point.0).is_strictly_positive() {
                return None;
            }
        }

        Some(point.clone())
    }
}

impl Intersection<Polygon> for Point2D {
    type Output = Point2D;

    #[inline(always)]
    fn intersection(&self, polygon: &Polygon) -> Option<Self::Output> {
        polygon.intersection(self)
    }
}
