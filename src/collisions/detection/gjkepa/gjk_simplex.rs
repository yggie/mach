#[cfg(test)]
#[path="../../../../tests/collisions/detection/gjkepa/gjk_simplex_test.rs"]
mod tests;

use maths::{Approximations, CrossProduct, DotProduct, Vec3D};
use collisions::shapes::Plane;

#[derive(Clone, Debug)]
pub struct GJKSimplex {
    vertices: [Vec3D; 4],
}

impl GJKSimplex {
    pub fn from_vertices(vertex_0: Vec3D, vertex_1: Vec3D, vertex_2: Vec3D, vertex_3: Vec3D) -> Result<GJKSimplex, GJKSimplexError> {
        let normal = (vertex_1 - vertex_0).cross(vertex_2 - vertex_0).normalize();
        let plane = Plane::new(vertex_0, normal);

        if plane.normal_projection_of(vertex_3).is_approximately_zero() {
            return Err(GJKSimplexError::CoplanarPoints);
        }

        Ok(GJKSimplex {
            vertices: [
                vertex_0,
                vertex_1,
                vertex_2,
                vertex_3,
            ],
        })
    }

    pub fn centroid(&self) -> Vec3D {
        0.25 * self.vertices.iter()
            .fold(Vec3D::zero(), |total, vertex| total + vertex)
    }

    #[inline(always)]
    pub fn vertices(&self) -> &[Vec3D; 4] {
        &self.vertices
    }

    #[inline(always)]
    pub fn vertex(&self, index: usize) -> &Vec3D {
        &self.vertices[index]
    }

    #[inline(always)]
    pub fn vertex_mut(&mut self, index: usize) -> &mut Vec3D {
        &mut self.vertices[index]
    }

    pub fn separating_planes_with_index_of_out_of_plane_point_iter<'a>(&'a self) -> Box<Iterator<Item=(usize, Plane)> + 'a> {
        let centroid = self.centroid();

        let iterator = INDEX_PERMUTATIONS.iter()
            .map(move |&indices| {
                let vertices = (
                    self.vertices[indices.1],
                    self.vertices[indices.2],
                    self.vertices[indices.3],
                );
                let edge_01 = vertices.1 - vertices.0;
                let edge_12 = vertices.2 - vertices.1;
                let normal_guess = edge_01.cross(edge_12).normalize();

                let reference_offset = centroid - vertices.0;
                let normal = if normal_guess.dot(reference_offset) > 0.0 {
                    -normal_guess
                } else {
                    normal_guess
                };

                return (indices.0, Plane::new(vertices.0, normal));
            });

        return Box::new(iterator);
    }
}

pub enum GJKSimplexError {
    CoplanarPoints,
}

static INDEX_PERMUTATIONS: [(usize, usize, usize, usize); 4] = [
    (0, 1, 2, 3),
    (1, 2, 3, 0),
    (2, 3, 0, 1),
    (3, 0, 1, 2),
];
