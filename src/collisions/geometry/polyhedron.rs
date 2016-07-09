#[cfg(test)]
#[path="../../../tests/collisions/geometry/polyhedron_test.rs"]
mod tests;

use maths::{ApproxEq, Approximations, CrossProduct, DotProduct, UnitVec3D, Vec3D};
use utils::{compute_surfaces_for_convex_hull, UnitVec3DGenerator};
use collisions::geometry::{ConvexHull3D, Face, Plane};

#[derive(Debug)]
pub struct Polyhedron {
    convex_hull: ConvexHull3D,
    triangulated_faces: Vec<[usize; 3]>,
}

impl Polyhedron {
    pub fn convex_hull_using_generator<F>(initial_vertices: &[Vec3D], point_generator: F) -> Result<Polyhedron, PolyhedronError> where F: Fn(UnitVec3D) -> Vec3D {
        // TODO unhandled edge cases:
        // 1. run out of points to generate
        let mut counter = 0;
        let mut generator = UnitVec3DGenerator::new();
        let mut vertices = Vec::from(initial_vertices);

        while counter < 1000 {
            match Polyhedron::convex_hull(&vertices) {
                Ok(polyhedron) => return Ok(polyhedron),

                Err(PolyhedronError::NotEnoughPoints) |
                Err(PolyhedronError::CoplanarPoints) => (),

                // Err(other_errors) => return Err(other_errors),
            }

            let next_direction = generator.next();
            let next_point = point_generator(next_direction);

            if !vertices.iter().any(|point| point.approx_eq(next_point)) {
                vertices.push(next_point);
            }

            counter = counter + 1;
        }

        panic!("Took more than 1000 iterations to construct the convex hull");
    }

    pub fn convex_hull(vertices: &[Vec3D]) -> Result<Polyhedron, PolyhedronError> {
        try!(validate_enough_points(vertices));
        try!(validate_points_are_not_coplanar(vertices));

        // TODO bit unsafe, but we trust the vertices are part of the convex
        // hull
        let vertices_clone = Vec::from(vertices);

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

        Ok(Polyhedron {
            convex_hull: ConvexHull3D::new(vertices_clone),
            triangulated_faces: triangulated_faces,
        })
    }

    #[inline(always)]
    pub fn vertices(&self) -> &Vec<Vec3D> {
        self.convex_hull.vertices()
    }

    pub fn faces_iter<'a>(&'a self) -> Box<Iterator<Item=Face<'a>> + 'a> {
        let iterator = self.triangulated_faces.iter()
            .map(move |indices| {
                Face::counter_clockwise_from(
                    self.convex_hull.vertices(),
                    indices.clone(),
                )
            });

        return Box::new(iterator);
    }

    pub fn add_vertex(&mut self, vertex: Vec3D) -> bool {
        let mut vertices = Vec::<Vec3D>::from(self.convex_hull.clone());
        vertices.push(vertex);

        let polyhedron = Polyhedron::convex_hull(&vertices)
            .expect("expected a valid polyhedron from an insertion to an already valid polyhedron");

        let new_vertex_was_accepted = polyhedron.vertices().iter()
            .find(|new_vertex| **new_vertex == vertex)
            .is_some();

        if new_vertex_was_accepted {
            self.convex_hull = polyhedron.convex_hull;
            self.triangulated_faces = polyhedron.triangulated_faces;

            return true;
        } else {
            return false;
        }
    }
}

#[derive(Debug)]
pub enum PolyhedronError {
    CoplanarPoints,
    NotEnoughPoints,
}

fn validate_enough_points(vertices: &[Vec3D]) -> Result<(), PolyhedronError> {
    if vertices.len() <= 3 {
        return Err(PolyhedronError::NotEnoughPoints);
    }

    Ok(())
}

fn validate_points_are_not_coplanar(vertices: &[Vec3D]) -> Result<(), PolyhedronError> {
    let mut remaining_points = vertices.iter();
    let first = remaining_points.next().unwrap();
    let second = remaining_points.next().unwrap();
    let third = remaining_points.next().unwrap();

    let normal = (second - first).cross(third - first).normalize();
    let plane = Plane::new(first.clone(), normal);

    for &point in remaining_points {
        if !plane.normal_projection_of(point).is_approximately_zero() {
            return Ok(())
        }
    }

    Err(PolyhedronError::CoplanarPoints)
}
