#[cfg(test)]
#[path="../../../tests/collisions/geometry/convex_polyhedron_test.rs"]
mod tests;

use maths::{CrossProduct, DotProduct, Vec3D};
use utils::{is_coplanar, compute_surfaces_for_convex_hull};
use collisions::geometry::Face;

#[derive(Debug)]
pub struct ConvexPolyhedron {
    vertices: Vec<Vec3D>,
    triangulated_faces: Vec<[usize; 3]>,
}

impl ConvexPolyhedron {
    pub unsafe fn from_triangulation(vertices: Vec<Vec3D>, triangulated_faces: Vec<[usize; 3]>) -> ConvexPolyhedron {
        ConvexPolyhedron {
            vertices: vertices,
            triangulated_faces: triangulated_faces,
        }
    }

    // TODO inputs tend to be already convex hulls, so how can we avoid the cost
    // of the expensive convex hull check for most cases?
    pub fn from_vertices(vertices: &[Vec3D]) -> Result<ConvexPolyhedron, ConvexPolyhedronError> {
        try!(validate_enough_points(vertices));
        try!(validate_points_are_not_coplanar(vertices));

        // TODO bit unsafe, but we trust the vertices are part of the convex
        // hull
        let vertices_clone = Vec::from(vertices);

        // TODO move this to the convex hull computation?
        let surfaces = compute_surfaces_for_convex_hull(&vertices_clone);
        let triangulated_faces = surfaces.iter()
            .map(|surface| {
                let vertex_0 = vertices[surface.nodes[0]];
                let vertex_1 = vertices[surface.nodes[1]];
                let vertex_2 = vertices[surface.nodes[2]];

                let counter_clockwise_normal = (vertex_2 - vertex_1).cross(vertex_0 - vertex_1).normalize();

                if counter_clockwise_normal.dot(surface.normal).is_sign_positive() {
                    return surface.nodes;
                } else {
                    return [surface.nodes[2], surface.nodes[1], surface.nodes[0]];
                }
            })
            .collect::<Vec<[usize; 3]>>();

        Ok(ConvexPolyhedron {
            vertices: vertices_clone,
            triangulated_faces: triangulated_faces,
        })
    }

    #[inline(always)]
    pub fn vertices(&self) -> &Vec<Vec3D> {
        &self.vertices
    }

    pub fn vertices_iter<'a>(&'a self) -> Box<Iterator<Item=&Vec3D> + 'a> {
        Box::new(self.vertices().iter())
    }

    pub fn faces_iter<'a>(&'a self) -> Box<Iterator<Item=Face<'a>> + 'a> {
        let iterator = self.triangulated_faces.iter()
            .map(move |indices| {
                Face::counter_clockwise_from(
                    self.vertices(),
                    indices.clone(),
                )
            });

        return Box::new(iterator);
    }
}

#[derive(Debug)]
pub enum ConvexPolyhedronError {
    CoplanarPoints,
    NotEnoughPoints,
}

fn validate_enough_points(vertices: &[Vec3D]) -> Result<(), ConvexPolyhedronError> {
    if vertices.len() <= 3 {
        return Err(ConvexPolyhedronError::NotEnoughPoints);
    }

    Ok(())
}

fn validate_points_are_not_coplanar(vertices: &[Vec3D]) -> Result<(), ConvexPolyhedronError> {
    if is_coplanar(vertices) {
        Err(ConvexPolyhedronError::CoplanarPoints)
    } else {
        Ok(())
    }
}
