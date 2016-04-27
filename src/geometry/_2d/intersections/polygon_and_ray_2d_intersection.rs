use geometry::Intersection;
use geometry::_2d::{Point2D, Polygon, Ray2D};

impl Intersection<Polygon> for Ray2D {
    type Output = Point2D;

    fn intersection(&self, polygon: &Polygon) -> Option<Self::Output> {
        let ray = self;

        polygon.intersection(ray.source() as &Point2D).or_else(|| {
            for (edge, plane) in polygon.separating_edges_and_planes_iter() {
                if plane.projection_along_normal(ray.source()).is_above_plane() {
                    if let Some(point) = edge.as_line().intersection(ray) {
                        return Some(point);
                    }
                }
            }

            None
        })
    }
}

impl Intersection<Ray2D> for Polygon {
    type Output = Point2D;

    #[inline(always)]
    fn intersection(&self, ray: &Ray2D) -> Option<Self::Output> {
        ray.intersection(self)
    }
}
