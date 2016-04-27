#[cfg(test)]
#[path="../../../../tests/geometry/_2d/intersections/ray_and_ray_2d_intersection_test.rs"]
mod tests;

use TOLERANCE;
use geometry::Intersection;
use geometry::_2d::{Point2D, Ray2D};

impl Intersection<Ray2D> for Ray2D {
    type Output = Point2D;

    fn intersection(&self, other_ray: &Ray2D) -> Option<Self::Output> {
        let rays = (self, other_ray);
        let sources = (rays.0.source(), rays.1.source());
        let directions = (rays.0.direction().vec().clone(), rays.1.direction().vec().clone());

        let denominator = directions.0.x * directions.1.y - directions.0.y * directions.1.x;

        if denominator.abs() < TOLERANCE {
            None
        } else {
            let numerator = directions.0.y * (sources.1.x - sources.0.x) - directions.0.x * (sources.1.y - sources.0.y);
            let factor_1 = numerator / denominator;

            Some(Point2D(sources.1 + factor_1 * directions.1))
        }
    }
}
