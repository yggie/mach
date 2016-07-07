#[cfg(test)]
#[path="../../../../../tests/collisions/geometry/_2d/intersections/polygon_and_polygon_2d_intersection_test.rs"]
mod tests;

use maths::_2d::Vec2D;
use collisions::geometry::Intersection;
use collisions::geometry::_2d::{Line2D, Point2D, Polygon};

impl Intersection<Polygon> for Polygon {
    type Output = Polygon;

    fn intersection(&self, other_polygon: &Polygon) -> Option<Self::Output> {
        let self_points_in_other: Vec<&Vec2D> = self.points().iter()
            .filter(|&&point| other_polygon.contains_point(point))
            .collect();

        let other_points_in_self: Vec<&Vec2D> = other_polygon.points().iter()
            .filter(|&&point| self.contains_point(point))
            .collect();

        let result = match (other_points_in_self.len(), self_points_in_other.len()) {
            (0, 0) => {
                let intersections: Vec<Vec2D> = self.edges_iter()
                    .map(Line2D::from)
                    .flat_map(|line| {
                        other_polygon.edges_iter()
                            .map(Line2D::from)
                            .filter_map(|other_line| line.intersection(&other_line))
                            .collect::<Vec<Point2D>>()
                    })
                    .map(Vec2D::from)
                    .collect();

                if intersections.len() == 0 {
                    None
                } else {
                    Some(Polygon::convex_hull_from_points(&intersections).unwrap())
                }
            },

            (0, _) => Some(self.clone()),

            (_, 0) => Some(other_polygon.clone()),

            _otherwise => {
                let intersections: Vec<Vec2D> = self.edges_iter()
                    .map(Line2D::from)
                    .flat_map(|line| {
                        other_polygon.edges_iter()
                            .map(Line2D::from)
                            .filter_map(|other_line| line.intersection(&other_line))
                            .collect::<Vec<Point2D>>()
                    })
                    .map(Vec2D::from)
                    .chain(other_points_in_self.iter().map(|&&p| p))
                    .chain(self_points_in_other.iter().map(|&&p| p))
                    .collect();

                if intersections.len() == 0 {
                    None
                } else {
                    Some(Polygon::convex_hull_from_points(&intersections).unwrap())
                }
            },
        };

        return result;
    }
}
