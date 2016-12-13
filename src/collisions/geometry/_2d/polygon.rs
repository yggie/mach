#[cfg(test)]
#[path="../../../../tests/collisions/geometry/_2d/polygon_test.rs"]
mod tests;

use std::collections::HashSet;

use {PI, Scalar};
use maths::{Approximations, DotProduct};
use maths::_2d::{UnitVec2D, Vec2D};
use collisions::geometry::Geometry;
use collisions::geometry::_2d::{Edge2D, Plane2D};

#[derive(Clone, Debug)]
pub struct Polygon(Vec<Vec2D>);

impl Geometry for Polygon {}

impl Polygon {
    pub fn convex_hull_from_points(original_points: &Vec<Vec2D>) -> Result<Polygon, ()> {
        if original_points.len() < 3 {
            return Err(());
        }

        let mut index_set = HashSet::new();
        let mut radians = 0.0;
        while index_set.len() < 3 {
            radians += 0.95 * PI / 4.0;
            let normal = UnitVec2D::from_radians(radians);
            let index = index_of_furthest_along(original_points, &normal);
            index_set.insert(index);
        }

        let indices: Vec<usize> = index_set.iter().cloned().collect();
        let centroid = indices.iter()
            .fold(Vec2D::zero(), |total, &index| {
                total + original_points[index].clone()
            }) * (1.0 / 3.0);

        let idx = (indices[0], indices[1], indices[2]);
        let mut edges: Vec<Edge2D> = vec!(
            Edge2D::new(&original_points[idx.0], &original_points[idx.1]),
            Edge2D::new(&original_points[idx.1], &original_points[idx.2]),
            Edge2D::new(&original_points[idx.2], &original_points[idx.0]),
        ).into_iter().map(|edge| {
            let plane = edge.counter_clockwise_plane();

            if plane.normal_projection_of(&centroid).is_strictly_positive() {
                edge.reversed()
            } else {
                edge
            }
        }).collect();

        while let Some(edge) = edges.pop() {
            let plane = edge.counter_clockwise_plane();
            let index = index_of_furthest_along(original_points, &plane.normal());

            if plane.normal_projection_of(&original_points[index]).is_strictly_positive() {
                index_set.insert(index);
                let point = &original_points[index];
                edges.push(Edge2D::new(edge.start, point));
                edges.push(Edge2D::new(point, edge.end));
            }
        }

        let mut sorted_points: Vec<Vec2D> = index_set.into_iter()
            .map(|index| original_points[index].clone())
            .collect();

        sorted_points.sort_by(|a, b| {
            let radians_a = (a.y - centroid.y).atan2(a.x - centroid.x);
            let radians_b = (b.y - centroid.y).atan2(b.x - centroid.x);

            radians_b.partial_cmp(&radians_a).unwrap()
        });

        Ok(Polygon(sorted_points))
    }

    #[inline]
    pub fn point(&self, index: usize) -> &Vec2D {
        &self.0[index]
    }

    #[inline]
    pub fn points(&self) -> &Vec<Vec2D> {
        &self.0
    }

    pub fn contains_point(&self, point: Vec2D) -> bool {
        self.separating_planes_iter()
            .all(|plane| !plane.normal_projection_of(&point).is_strictly_positive())
    }

    pub fn edges_iter<'a>(&'a self) -> Box<Iterator<Item=Edge2D<'a>> + 'a> {
        let next_points = self.0.iter().skip(1).chain(self.0.iter().take(1));
        let points = self.0.iter();
        let iterator = points.zip(next_points)
            .map(|(a, b)| Edge2D::new(a, b));

        Box::new(iterator)
    }

    pub fn separating_planes_iter<'a>(&'a self) -> Box<Iterator<Item=Plane2D> + 'a> {
        Box::new(self.edges_iter().map(|edge| edge.counter_clockwise_plane()))
    }

    pub fn separating_edges_and_planes_iter<'a>(&'a self) -> Box<Iterator<Item=(Edge2D<'a>, Plane2D)> + 'a> {
        Box::new(self.edges_iter().map(|edge| {
            let plane = edge.counter_clockwise_plane();

            (edge, plane)
        }))
    }
}

fn index_and_projection_of_furthest_along(points: &Vec<Vec2D>, normal: &UnitVec2D) -> (usize, Scalar) {
    let initial_projection = normal.dot(&points[0]);

    return points.iter()
        .enumerate()
        .skip(1)
        .fold((0, initial_projection), |(index_of_max, max_projection), (i, point)| {
            match normal.dot(point) {
                x if x > max_projection => (i, x),
                _otherwise => (index_of_max, max_projection),
            }
        });
}

fn index_of_furthest_along(points: &Vec<Vec2D>, normal: &UnitVec2D) -> usize {
    let (index, _projection) = index_and_projection_of_furthest_along(points, normal);

    return index;
}

impl From<Polygon> for Vec<Vec2D> {
    fn from(polygon: Polygon) -> Vec<Vec2D> {
        polygon.0
    }
}
