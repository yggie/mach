use { Scalar, TOLERANCE };
use maths::Vect;

/// Represents a `Surface` for a given set of points.
#[derive(Copy, Clone)]
pub struct Surface {
    /// The `Surface` normal
    pub normal: Vect,
    /// The node indices associated with the `Surface`
    pub nodes: [usize; 3],
}

impl Surface {
    /// Creates a new `Surface` from the point cloud and indices provided.
    pub fn new(vertices: &Vec<Vect>, index_0: usize, index_1: usize, index_2: usize) -> Surface {
        let reference_point = vertices.iter()
            .fold(Vect::new_zero(), |total, &vector| {
                total + vector
            }) / (vertices.len() as Scalar);

        let base = vertices[index_0];
        let relative_to_reference = base - reference_point;
        let edge_0 = vertices[index_1] - base;
        let edge_1 = vertices[index_2] - base;
        let mut normal = edge_0.cross(edge_1).normalize();

        if normal.dot(relative_to_reference) < TOLERANCE {
            normal = -normal;
        }

        return Surface {
            normal: normal,
            nodes: [index_0, index_1, index_2],
        };
    }

    /// Computes the centroid of a `Surface` using the node indices in the
    /// `Surface` and the point cloud provided.
    pub fn compute_centroid(surface: &Surface, vertices: &Vec<Vect>) -> Vect {
        return surface.nodes.iter()
            .fold(Vect::new_zero(), |total, &index| {
                total + vertices[index]
            }) / 3.0;
    }
}
