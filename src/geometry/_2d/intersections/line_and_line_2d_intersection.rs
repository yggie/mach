use geometry::Intersection;
use geometry::_2d::{Line2D, Point2D};

impl Intersection<Line2D> for Line2D {
    type Output = Point2D;

    fn intersection(&self, other_line: &Line2D) -> Option<Self::Output> {
        let self_as_ray = self.as_ray();

        self_as_ray.intersection(other_line)
            .and_then(|point| self.intersection(&point))
    }
}
