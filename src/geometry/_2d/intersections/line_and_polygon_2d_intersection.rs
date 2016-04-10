use geometry::Intersection;
use geometry::_2d::{Line2D, Polygon};

impl Intersection<Line2D> for Polygon {
    type Output = Line2D;

    fn intersection(&self, line: &Line2D) -> Option<Self::Output> {
        let polygon = self;

        return line.as_ray().intersection(polygon).and_then(|intersection_0| {
            line.as_ray_from_end().reversed().intersection(polygon)
                .map(move |intersection_1| (intersection_0.0, intersection_1.0))
        }).map(|intersections| {
            return Line2D::new(intersections.0, intersections.1);
        });
    }
}
