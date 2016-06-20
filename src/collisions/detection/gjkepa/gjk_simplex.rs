use maths::{CrossProduct, DotProduct, Vec3D};
use geometry::Plane;

#[derive(Clone, Debug)]
pub struct GJKSimplex {
    pub vertices: [Vec3D; 4],
}

impl GJKSimplex {
    pub fn new(vertex_0: Vec3D, vertex_1: Vec3D, vertex_2: Vec3D, vertex_3: Vec3D) -> GJKSimplex {
        GJKSimplex {
            vertices: [
                vertex_0,
                vertex_1,
                vertex_2,
                vertex_3,
            ],
        }
    }

    pub fn centroid(&self) -> Vec3D {
        0.25 * self.vertices.iter()
            .fold(Vec3D::zero(), |total, vertex| total + vertex)
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

static INDEX_PERMUTATIONS: [(usize, usize, usize, usize); 4] = [
    (0, 1, 2, 3),
    (1, 2, 3, 0),
    (2, 3, 0, 1),
    (3, 0, 1, 2),
];
