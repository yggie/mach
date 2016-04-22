use maths::Vec3D;

use geometry::Plane;

use maths::{CrossProduct, DotProduct};
use super::simplex_cache::SimplexCache;
use super::minkowski_difference::{MinkowskiDifference, IndexPair};

static SURFACE_INDICES_COMBINATIONS: [(usize, usize, usize); 4] = [
    (1, 2, 3),
    (0, 2, 3),
    (0, 1, 3),
    (0, 1, 2),
];

#[derive(Clone)]
pub struct Simplex<'a> {
    pub diff: MinkowskiDifference<'a>,
    pub support_points: [(Vec3D, IndexPair); 4],
}

impl<'a> Simplex<'a> {
    pub fn new(cache: &SimplexCache, diff: MinkowskiDifference<'a>) -> Simplex<'a> {
        let support_points = [
            (diff.vertex(&cache.index_pairs()[0]), cache.index_pairs()[0]),
            (diff.vertex(&cache.index_pairs()[1]), cache.index_pairs()[1]),
            (diff.vertex(&cache.index_pairs()[2]), cache.index_pairs()[2]),
            (diff.vertex(&cache.index_pairs()[3]), cache.index_pairs()[3]),
        ];

        return Simplex {
            diff: diff,
            support_points: support_points,
        };
    }

    pub fn centroid(&self) -> Vec3D {
        self.support_points.iter()
            .fold(Vec3D::zero(), |total, &(vertex, _index_pair)| {
                total + vertex
            }) / 4.0
    }

    pub fn surfaces_iter<'b>(&'b self) -> Box<Iterator<Item=(Plane, (usize, usize, usize))> + 'b> {
        let centroid = self.centroid();

        let iterator = SURFACE_INDICES_COMBINATIONS.iter()
            .map(move |&indices| {
                let vertices = (
                    self.support_points[indices.0].0,
                    self.support_points[indices.1].0,
                    self.support_points[indices.2].0,
                );
                let edge_01 = vertices.1 - vertices.0;
                let edge_12 = vertices.2 - vertices.1;
                let guess = edge_01.cross(edge_12).normalize();

                let reference_offset = centroid - vertices.0;
                let normal = if guess.dot(reference_offset) > 0.0 {
                    -guess
                } else {
                    guess
                };

                return (Plane::new(vertices.0, normal), indices);
            });

        return Box::new(iterator);
    }
}
