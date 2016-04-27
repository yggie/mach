use TOLERANCE;
use geometry::Intersection;
use geometry::_2d::{Point2D, Ray2D};

impl Intersection<Point2D> for Ray2D {
    type Output = Point2D;

    fn intersection(&self, point: &Point2D) -> Option<Self::Output> {
        let ray = self;

        let perpendicular_plane = ray.counter_clockwise_plane();

        if perpendicular_plane.projection_along_normal(&point.0).is_on_plane() {
            let projection = ray.project_along_direction(&point.0);

            if projection > -TOLERANCE {
                Some(point.clone())
            } else {
                None
            }
        } else {
            None
        }
    }
}

impl Intersection<Ray2D> for Point2D {
    type Output = Point2D;

    #[inline(always)]
    fn intersection(&self, ray: &Ray2D) -> Option<Self::Output> {
        ray.intersection(self)
    }
}
