use collisions::shapes::Intersection;
use collisions::shapes::_2d::{Line2D, Point2D};

impl Intersection<Line2D> for Point2D {
    type Output = Point2D;

    fn intersection(&self, line: &Line2D) -> Option<Self::Output> {
        let point = self;

        line.as_ray().intersection(point).and_then(|intersection_point| {
            if line.projection_of(&intersection_point.0).is_on_line() {
                Some(intersection_point)
            } else {
                None
            }
        })
    }
}

impl Intersection<Point2D> for Line2D {
    type Output = Point2D;

    fn intersection(&self, point: &Point2D) -> Option<Self::Output> {
        point.intersection(self)
    }
}
