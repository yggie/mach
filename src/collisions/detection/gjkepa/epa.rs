#[cfg(test)]
#[path="../../../../tests/collisions/detection/gjkepa/epa_test.rs"]
mod tests;

use {NEG_INFINITY, Scalar};
use maths::{ApproxEq, Approximations, CoordinateTransform, Vec3D};
use maths::_2d::Vec2D;
use utils::compute_surfaces_for_convex_hull;
use algorithms::IterativeAlgorithm;
use collisions::{CollisionData, ContactSet, SupportMap};
use collisions::shapes::{Direction, Intersection, Plane, Ray};
use collisions::shapes::_2d::{Line2D, Polygon};
use collisions::shapes::convex_shapes::ConvexPolyhedron;
use collisions::detection::gjkepa::{GJKSimplex, MinkowskiDifference};

pub struct EPA<'a> {
    diff: MinkowskiDifference<'a>,
    vertices: Vec<Vec3D>,
    has_converged: bool,
    triangulated_faces: Vec<[usize; 3]>,
}

impl<'a> EPA<'a> {
    pub fn new(simplex: &GJKSimplex, data_0: &'a CollisionData, data_1: &'a CollisionData) -> EPA<'a> {
        let diff = MinkowskiDifference(data_0, data_1);
        let vertices: Vec<Vec3D> = simplex.vertices().iter().cloned().collect();
        let triangulated_faces_original = [
            [0, 1, 2],
            [0, 1, 3],
            [0, 2, 3],
            [1, 2, 3],
        ];

        let centroid = 0.25 * vertices.iter()
            .fold(Vec3D::zero(), |total, vertex| total + vertex);
        let triangulated_faces: Vec<[usize; 3]> = triangulated_faces_original.iter()
            .map(|triangulation| {
                let plane = Plane::from_counter_clockwise_points(
                    vertices[triangulation[0]],
                    vertices[triangulation[1]],
                    vertices[triangulation[2]],
                );

                if plane.normal_projection_of(centroid) < 0.0 {
                    *triangulation
                } else {
                    [triangulation[0], triangulation[2], triangulation[1]]
                }
            })
            .collect();

        EPA {
            diff: diff,
            vertices: vertices,
            has_converged: false,
            triangulated_faces: triangulated_faces,
        }
    }
}

impl<'a> IterativeAlgorithm for EPA<'a> {
    type Result = EPAPolyhedron<'a>;

    fn result(self) -> Self::Result {
        EPAPolyhedron {
            diff: self.diff,
            polyhedron: unsafe {
                ConvexPolyhedron::from_triangulation(self.vertices, self.triangulated_faces)
            },
        }
    }

    fn has_converged(&self) -> bool {
        self.has_converged
    }

    fn next_iteration(&mut self) {
        if self.has_converged {
            return;
        }

        // TODO the triangulated faces never change order, so we can skip work
        // by filtered values in the next iteration
        let candidate = self.triangulated_faces.iter()
            .filter_map(|triangulation| {
                let plane = Plane::from_counter_clockwise_points(
                    self.vertices[triangulation[0]],
                    self.vertices[triangulation[1]],
                    self.vertices[triangulation[2]],
                );

                // TODO this should only return points on the boundary support
                // points
                self.diff.support_points_iter(Direction::from(plane.normal()))
                    .filter(|&point| {
                        // I don’t know why, but the uniqueness test is still
                        // required
                        !self.vertices.iter().any(|vertex| vertex.approx_eq(point)) &&
                            plane.normal_projection_of(point).is_strictly_positive()
                    })
                    .next()
            })
            .next();

        match candidate {
            Some(new_support_point) => {
                self.vertices.push(new_support_point);

                let mid_point = self.vertices.iter()
                    .fold(Vec3D::zero(), |total, vertex| {
                        total + vertex
                    }) / self.vertices.len() as Scalar;

                let surfaces = compute_surfaces_for_convex_hull(&self.vertices);
                self.triangulated_faces = surfaces.into_iter()
                    .map(|surface| {
                        let mut node_guess = surface.nodes;

                        let plane = Plane::from_counter_clockwise_points(
                            self.vertices[node_guess[0]],
                            self.vertices[node_guess[1]],
                            self.vertices[node_guess[2]],
                        );

                        if plane.normal_projection_of(mid_point).is_strictly_positive() {
                            node_guess = [node_guess[0], node_guess[2], node_guess[1]];
                        }

                        node_guess
                    })
                    .collect();
            },

            None => self.has_converged = true,
        }
    }
}

pub struct EPAPolyhedron<'a> {
    diff: MinkowskiDifference<'a>,
    polyhedron: ConvexPolyhedron,
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
            let vertices = self.diff.0.support_points_iter(-Direction::from(contact_normal))
                .collect::<Vec<Vec3D>>();

            Feature::from_vertices(vertices)
        };

        let feature_1 = {
            let vertices = self.diff.1.support_points_iter( Direction::from(contact_normal))
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
                let ray_0 = Ray::from_points(vertex_00, vertex_01);
                let ray_1 = Ray::from_points(vertex_10, vertex_11);

                // TODO does this need a depth correction?
                let contact_point = Ray::closest_point_to_rays(&ray_0, &ray_1);

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
            Plane::new(contact_point_0, contact_plane.normal()),
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
            Plane::new(points[0], contact_plane.normal()),
            points,
        );
    }

    pub fn polyhedron(&self) -> &ConvexPolyhedron {
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
