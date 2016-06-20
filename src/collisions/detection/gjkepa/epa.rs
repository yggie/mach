#[cfg(test)]
#[path="../../../../tests/collisions/detection/gjkepa/epa_test.rs"]
mod tests;

use {NEG_INFINITY, Scalar};
use maths::{Approximations, CoordinateTransform, CrossProduct, DotProduct, UnitVec3D, Vec3D};
use maths::_2d::Vec2D;
use utils::compute_surfaces_for_convex_hull;
use geometry::{Intersection, Line, Plane};
use geometry::_2d::{Line2D, Polygon};
use detection::ContactSet;
use algorithms::IterativeAlgorithm;
use collisions::{BasicCollisionData, SupportMap};
use collisions::detection::gjkepa::{GJKSimplex, MinkowskiDifference};

pub struct EPA<'a> {
    diff: MinkowskiDifference<'a>,
    polyhedron: Polyhedron,
    has_converged: bool,
}

impl<'a> EPA<'a> {
    pub fn new(simplex: &GJKSimplex, data_0: &'a BasicCollisionData, data_1: &'a BasicCollisionData) -> EPA<'a> {
        EPA {
            diff: MinkowskiDifference(data_0, data_1),
            polyhedron: Polyhedron::convex_hull(&simplex.vertices),
            has_converged: false,
        }
    }
}

impl<'a> IterativeAlgorithm for EPA<'a> {
    type Result = EPAPolyhedron<'a>;

    fn result(self) -> Self::Result {
        EPAPolyhedron {
            diff: self.diff,
            polyhedron: self.polyhedron,
        }
    }

    fn has_converged(&self) -> bool {
        self.has_converged
    }

    fn next_iteration(&mut self) {
        if self.has_converged {
            return;
        }

        let candidate_point = self.polyhedron.faces_iter()
            .filter_map(|face| {
                let new_support_point = self.diff.support_point(Vec3D::from(face.normal()));

                if face.normal_projection_of(new_support_point).is_strictly_positive() {
                    Some(new_support_point)
                } else {
                    None
                }
            })
            .next();

        match candidate_point {
            Some(vertex) => {
                self.polyhedron.add_vertex(vertex);
            },

            None => self.has_converged = true,
        }
    }
}

pub struct EPAPolyhedron<'a> {
    diff: MinkowskiDifference<'a>,
    polyhedron: Polyhedron,
}

impl<'a> EPAPolyhedron<'a> {
    pub fn compute_contact_set(self) -> ContactSet {
        let mut iterator = self.polyhedron.faces_iter();
        let initial_face = iterator.next()
            .expect("expected polyhedron to have at least one face, but it did not");

        let (penetration_depth, closest_face) = iterator
            .fold((NEG_INFINITY, initial_face), |(origin_to_closest_face_offset, closest_face), face| {
                let offset = face.normal_projection_of_origin();
                let diff = offset - origin_to_closest_face_offset;

                if diff.is_strictly_positive() {
                    (offset, face)
                } else {
                    (origin_to_closest_face_offset, closest_face)
                }
            });

        let contact_normal = -closest_face.normal();
        let feature_0 = {
            let shape = self.diff.0.shape();

            let vertices = shape.support_points_iter( Vec3D::from(contact_normal))
                .collect::<Vec<Vec3D>>();

            Feature::from_vertices(vertices)
        };

        let feature_1 = {
            let shape = self.diff.1.shape();

            let vertices = shape.support_points_iter(-Vec3D::from(contact_normal))
                .collect::<Vec<Vec3D>>();

            Feature::from_vertices(vertices)
        };

        return match (feature_0, feature_1) {
            (Feature::Vertex(vertex), _other) => {
                let correction = contact_normal * penetration_depth / 2.0;
                let contact_point = vertex - correction;

                ContactSet::new(
                    Plane::new(contact_point - contact_normal * penetration_depth, contact_normal),
                    vec!(contact_point),
                )
            },

            (_other, Feature::Vertex(vertex)) => {
                let correction = contact_normal * penetration_depth / 2.0;
                let contact_point = vertex + correction;

                ContactSet::new(
                    Plane::new(contact_point - contact_normal * penetration_depth, contact_normal),
                    vec!(contact_point),
                )
            },

            (Feature::Edge(vertex_00, vertex_01), Feature::Edge(vertex_10, vertex_11)) => {
                let line_0 = Line::from_points(vertex_00, vertex_01);
                let line_1 = Line::from_points(vertex_10, vertex_11);

                // TODO does this need a depth correction?
                let contact_point = line_0.closest_point_to_line(&line_1);

                ContactSet::new(
                    Plane::new(contact_point - contact_normal * penetration_depth, contact_normal),
                    vec!(contact_point),
                )
            },

            (Feature::Edge(edge_vertex_0, edge_vertex_1), Feature::Face(face_vertices)) => {
                EPAPolyhedron::compute_contact_set_for_edge_face((edge_vertex_0, edge_vertex_1), face_vertices, Plane::from(closest_face))
            },

            (Feature::Face(face_vertices), Feature::Edge(edge_vertex_0, edge_vertex_1)) => {
                EPAPolyhedron::compute_contact_set_for_edge_face((edge_vertex_0, edge_vertex_1), face_vertices, Plane::from(closest_face).reversed())
            },

            (Feature::Face(face_vertices_0), Feature::Face(face_vertices_1)) => {
                EPAPolyhedron::compute_contact_set_for_face_face(face_vertices_0, face_vertices_1, Plane::from(closest_face))
            },
        };
    }

    fn compute_contact_set_for_edge_face(edge_vertices: (Vec3D, Vec3D), face_vertices: Vec<Vec3D>, contact_plane: Plane) -> ContactSet {
        let coordinates = CoordinateTransform::from_plane(&contact_plane);

        let start = coordinates.transform(edge_vertices.0);
        let end = coordinates.transform(edge_vertices.1);
        let average_z_0 = (start.z + end.z) / 2.0;
        let edge_points = Line2D::new(
            Vec2D::new(start.x, start.y),
            Vec2D::new(end.x, end.y),
        );

        let (polygon, average_z_1) = project_to_polygon_2d(face_vertices, &coordinates);
        let average_z = (average_z_0 + average_z_1) / 2.0;

        let intersection = polygon.intersection(&edge_points).unwrap();
        let contact_point_0 = coordinates.transform_with_inverse(Vec3D::new(intersection.start.x, intersection.start.y, average_z));
        let contact_point_1 = coordinates.transform_with_inverse(Vec3D::new(intersection.end.x, intersection.end.y, average_z));

        return ContactSet::new(
            Plane::new(contact_point_0, -contact_plane.normal()),
            vec!(contact_point_0, contact_point_1),
        );
    }

    fn compute_contact_set_for_face_face(face_vertices_0: Vec<Vec3D>, face_vertices_1: Vec<Vec3D>, contact_plane: Plane) -> ContactSet {
        let coordinates = CoordinateTransform::from_plane(&contact_plane);

        let (polygon_0, average_z_0) = project_to_polygon_2d(face_vertices_0, &coordinates);
        let (polygon_1, average_z_1) = project_to_polygon_2d(face_vertices_1, &coordinates);
        let average_z = (average_z_0 + average_z_1) / 2.0;

        let intersection = polygon_0.intersection(&polygon_1)
            .expect("expected an intersection for face-face features, but none was found");

        let points: Vec<Vec3D> = intersection.points().iter()
            .map(|point| coordinates.transform_with_inverse(Vec3D::new(point.x, point.y, average_z)))
            .collect();

        return ContactSet::new(
            Plane::new(points[0], -contact_plane.normal()),
            points,
        );
    }

    pub fn polyhedron(&self) -> &Polyhedron {
        &self.polyhedron
    }
}

fn project_to_polygon_2d(vertices: Vec<Vec3D>, coordinates: &CoordinateTransform) -> (Polygon, Scalar) {
    let points: Vec<Vec3D> = vertices.into_iter()
        .map(|vertex| coordinates.transform(vertex))
        .collect();

    let average_z = points.iter().fold(0.0, |total, point| total + point.z) / points.len() as Scalar;
    let flat_projected_points: Vec<Vec2D> = points.into_iter()
        .map(|point| Vec2D::new(point.x, point.y))
        .collect();

    let polygon = Polygon::convex_hull_from_points(&flat_projected_points)
        // TODO can we avoid needing to validate this?
        .expect("A valid face always has enough points");

    return (polygon, average_z);
}

enum Feature {
    Vertex(Vec3D),
    Edge(Vec3D, Vec3D),
    Face(Vec<Vec3D>),
}

impl Feature {
    fn from_vertices(colliding_vertices: Vec<Vec3D>) -> Feature {
        let length = colliding_vertices.len();

        match length {
            0 => panic!("no vertices found to describe the contact feature!"),
            1 => Feature::Vertex(colliding_vertices[0]),
            2 => Feature::Edge(colliding_vertices[0], colliding_vertices[1]),
            _otherwise => Feature::Face(colliding_vertices),
        }
    }
}

#[derive(Clone)]
pub struct Face<'a> {
    indices: [usize; 3],
    vertices: &'a [Vec3D],
}

impl<'a> Face<'a> {
    pub fn counter_clockwise_from(vertices: &'a [Vec3D], indices: [usize; 3]) -> Face<'a> {
        Face {
            indices: indices,
            vertices: vertices,
        }
    }

    #[inline(always)]
    pub fn normal(&self) -> UnitVec3D {
        let vertex_0 = self.vertices[self.indices[0]];
        let vertex_1 = self.vertices[self.indices[1]];
        let vertex_2 = self.vertices[self.indices[2]];

        (vertex_2 - vertex_1).cross(vertex_0 - vertex_1).normalize()
    }

    #[inline(always)]
    pub fn vertex(&self, index: usize) -> Vec3D {
        self.vertices[self.indices[index]]
    }

    pub fn indices(&self) -> [usize; 3] {
        self.indices.clone()
    }

    pub fn normal_projection_of(&self, point: Vec3D) -> Scalar {
        self.normal().dot(point - self.vertices[self.indices[0]])
    }

    pub fn normal_projection_of_origin(&self) -> Scalar {
        -self.normal().dot(self.vertices[self.indices[0]])
    }
}

impl<'a> From<Face<'a>> for Plane {
    fn from(face: Face<'a>) -> Plane {
        Plane::new(face.vertices[0], face.normal())
    }
}

#[derive(Clone)]
pub struct ConvexHull3D(Vec<Vec3D>);

impl ConvexHull3D {
    #[inline(always)]
    pub fn vertices(&self) -> &Vec<Vec3D> {
        &self.0
    }
}

impl From<ConvexHull3D> for Vec<Vec3D> {
    fn from(convex_hull: ConvexHull3D) -> Vec<Vec3D> {
        convex_hull.0
    }
}

pub struct Polyhedron {
    convex_hull: ConvexHull3D,
    triangulated_faces: Vec<[usize; 3]>,
}

impl Polyhedron {
    pub fn convex_hull(vertices: &[Vec3D]) -> Polyhedron {
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

        Polyhedron {
            convex_hull: ConvexHull3D(vertices_clone),
            triangulated_faces: triangulated_faces,
        }
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

        let polyhedron = Polyhedron::convex_hull(&vertices);

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
