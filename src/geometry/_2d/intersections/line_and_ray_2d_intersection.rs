use geometry::Intersection;
use geometry::_2d::{Line2D, Point2D, Ray2D};

impl Intersection<Ray2D> for Line2D {
    type Output = Point2D;

    fn intersection(&self, ray: &Ray2D) -> Option<Self::Output> {
        let edge = self;
        let edge_ray = edge.as_ray();

        ray.intersection(&edge_ray).and_then(|point| {
            edge.intersection(&point)
        })
    }
}

impl Intersection<Line2D> for Ray2D {
    type Output = Point2D;

    fn intersection(&self, edge: &Line2D) -> Option<Self::Output> {
        edge.intersection(self)
    }
}
