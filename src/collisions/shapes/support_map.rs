use std::iter;

use TOLERANCE;
use maths::{ApproxEq, Approximations, CrossProduct, DotProduct, Vec3D};
use utils::UnitVec3DGenerator;
use collisions::shapes::Direction;

pub trait SupportMap {
    fn support_points_iter<'a>(&'a self, direction: Direction) -> Box<Iterator<Item=Vec3D> + 'a>;

    fn unique_support_point(&self, direction: Direction) -> Option<Vec3D> {
        let mut iterator = self.support_points_iter(direction);
        let possible_unique_point = iterator.next().unwrap();
        let remaining_count = iterator.count();

        if remaining_count == 0 {
            Some(possible_unique_point)
        } else {
            None
        }
    }

    /// Returns support points which exist strictly on the boundary of the
    /// surface constructed from all support points projected on the plane
    /// described by the input direction
    fn boundary_support_points_iter<'a>(&'a self, direction: Direction) -> Box<Iterator<Item=Vec3D> + 'a> {
        let points: Vec<Vec3D> = self.support_points_iter(direction).collect();

        if points.len() <= 2 {
            return Box::new(points.into_iter());
        }

        let unit_direction = Vec3D::from(direction).normalize();
        let mut generator = UnitVec3DGenerator::new();
        let mut next_boundary_direction = || {
            generator.gen_next().cross(unit_direction)
        };
        let mut boundary_points: Vec<Vec3D> = Vec::with_capacity(points.len());

        while boundary_points.len() < 2 {
            let guess = next_boundary_direction();

            if let Some(unique_support_point) = self.unique_support_point(Direction::from(guess)) {
                if !boundary_points.iter().any(|point| point.approx_eq(unique_support_point)) {
                    boundary_points.push(unique_support_point);
                }
            }
        }

        let mut edges_to_test = vec!((0, 1), (1, 0));
        while let Some((a, b)) = edges_to_test.pop() {
            let point_a = boundary_points[a];
            let point_b = boundary_points[b];

            let edge_direction = (point_b - point_a).normalize();
            let edge_normal = edge_direction.cross(unit_direction);

            let mut support_points: Vec<Vec3D> = points.support_points_iter(Direction::from(edge_normal)).collect();
            let perp_distance_from_edge = (support_points[0] - point_a).dot(edge_normal);

            if !perp_distance_from_edge.is_strictly_positive() {
                break;
            }

            let mid_point = 0.5 * (point_a + point_b);
            let mid_to_a = point_a - mid_point;
            support_points.sort_by_key(move |point| {
                ((point - mid_point).dot(mid_to_a).asin() / TOLERANCE) as usize
            });

            let mut prev_index = a;
            for point in support_points.into_iter() {
                let new_index = boundary_points.len();
                boundary_points.push(point);

                edges_to_test.push((prev_index, new_index));
                prev_index = new_index;
            }

            edges_to_test.push((prev_index, b));
        }

        return Box::new(boundary_points.into_iter());
    }
}

#[cfg(test)]
#[path="../../../tests/collisions/shapes/vec_vec_3d_support_map_test.rs"]
mod vec_tests;

impl SupportMap for Vec<Vec3D> {
    fn support_points_iter<'a>(&'a self, input_direction: Direction) -> Box<Iterator<Item=Vec3D> + 'a> {
        let direction = Vec3D::from(input_direction);

        if self.len() > 0 {
            let mut iterator = self.iter();
            let mut points: Vec<Vec3D> = vec!(*iterator.next().unwrap());
            let mut furthest_distance = points[0].dot(direction);

            for &point in iterator {
                let distance = point.dot(direction);
                let diff = distance - furthest_distance;

                if diff.is_strictly_positive() {
                    furthest_distance = distance;
                    points = vec!(point);
                } else if diff.is_approximately_zero() {
                    points.push(point);
                }
            }

            return Box::new(points.into_iter());
        } else {
            Box::new(iter::empty())
        }
    }
}
