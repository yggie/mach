use std::rc::Rc;
use std::fmt;
use std::collections::HashMap;

use {Scalar, NEG_INFINITY, TOLERANCE};
use maths::{DotProduct, Matrix, Vec3D};
use collisions::geometry::shapes::{Shape, ShapeRef};

/// A `TriangleMesh` object represents a mesh of triangles, built from a set of
/// points and element connections.
#[derive(Clone, Debug)]
pub struct TriangleMesh {
    vertices: Rc<Vec<Vec3D>>,
    elements: Rc<Vec<(usize, usize, usize)>>,
    unique_nodes: Vec<usize>,
}

impl TriangleMesh {
    /// Constructs a new `TriangleMesh` using the provided vertices and element
    /// indices, stored as an indexed array of triangle elements.
    // TODO run validations to ensure the triangle mesh is not malformed.
    pub fn new(vertices: Rc<Vec<Vec3D>>, elements: Vec<(usize, usize, usize)>) -> TriangleMesh {
        let unique_nodes = elements.iter()
            .map(|&(i, j, k)| [i, j, k])
            .fold(HashMap::new(), |mut set, indices| {
                for index in indices.iter() {
                    if !set.contains_key(index) {
                        set.insert(*index, ());
                    }
                }
                set
            })
            .keys()
            .map(|&a| a)
            .collect();

        TriangleMesh {
            vertices: vertices,
            elements: Rc::new(elements),
            unique_nodes: unique_nodes,
        }
    }
}

impl fmt::Display for TriangleMesh {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let result = write!(f, "TriangleMesh{{");

        if result.is_err() {
            return result;
        }

        for &(i, j, k) in self.elements.iter() {
            let v0 = self.vertices[i];
            let v1 = self.vertices[j];
            let v2 = self.vertices[k];

            let result = write!(f, "({}, {}, {}),", v0, v1, v2);

            if result.is_err() {
                return result;
            }
        }

        return write!(f, "}}");
    }
}

impl Shape for TriangleMesh {
    fn downcast(&self) -> ShapeRef {
        ShapeRef::TriangleMesh(self)
    }

    fn volume(&self) -> Scalar {
        0.0
    }

    fn inertia(&self) -> Matrix {
        // TODO implement this properly (somehow?)
        Matrix::identity()
    }

    fn vertex(&self, index: usize) -> Vec3D {
        self.vertices[self.unique_nodes[index]]
    }

    fn vertices_len(&self) -> usize {
        self.unique_nodes.len()
    }

    fn vertices_iter<'a>(&'a self) -> Box<Iterator<Item=Vec3D> + 'a> {
        Box::new(self.unique_nodes.iter().map(move |&i| self.vertices[i]))
    }

    fn support_indices_for(&self, direction: Vec3D) -> Vec<usize> {
        // TODO iterate by face really
        self.vertices_iter()
            .enumerate()
            .fold(vec!((0, NEG_INFINITY)), |best_matches, (index, vertex)| {
                let max_dot_product = best_matches[0].1;
                let dot_product = vertex.dot(direction);

                if dot_product > max_dot_product + TOLERANCE {
                    vec!((index, dot_product))
                } else if (dot_product - max_dot_product).abs() < TOLERANCE {
                    let mut new_matches = vec!((index, dot_product));
                    for &item in best_matches.iter() {
                        new_matches.push(item);
                    }
                    new_matches
                } else {
                    best_matches
                }
            }).iter()
            .map(|&(index, _)| {
                index
            })
            .collect()
    }

    fn box_clone(&self) -> Box<Shape> {
        Box::new(self.clone())
    }
}
