use std::collections::HashSet;

use {NEG_INFINITY, Scalar, TOLERANCE};
use maths::{CoordinateTransform, Vec3D};
use maths::_2d::Vec2D;
use entities::Form;
use geometry::{Intersection, Line, Plane};
use geometry::_2d::{Line2D, Polygon};
use detection::ContactSet;

use super::simplex::Simplex;
use super::minkowski_difference::{MinkowskiDifference, IndexPair};

enum Feature {
    Vertex(usize),
    Edge((usize, usize)),
    Face(HashSet<usize>),
}

pub struct Polytope<'a> {
    pub diff: MinkowskiDifference<'a>,
    pub surfaces: Vec<(Plane, (usize, usize, usize))>,
    pub support_points: Vec<(Vec3D, IndexPair)>,
}

impl<'a> Polytope<'a> {
    pub fn from_simplex(simplex: Simplex<'a>) -> Polytope<'a> {
        let surfaces = simplex.surfaces_iter().collect();

        Polytope {
            diff: simplex.diff,
            surfaces: surfaces,
            support_points: simplex.support_points.clone().to_vec(),
        }
    }

    pub fn compute_contact_points(&self) -> ContactSet {
        let dummy_plane = Plane::new(Vec3D::new(1.0, 0.0, 0.0), Vec3D::new(1.0, 0.0, 0.0).into());

        let faces = vec!((0, 0, 0));
        let (penetration_depth, closest_plane, faces) = self.surfaces.iter().skip(1)
            .fold((NEG_INFINITY, dummy_plane, faces), |(origin_to_closest_plane_offset, closest_plane, mut faces), &(ref plane, face)| {
                let offset = plane.normal_projection_of_origin();
                let diff = offset - origin_to_closest_plane_offset;

                if diff > TOLERANCE {
                    (offset, plane.clone(), vec!(face))
                } else if diff.abs() < TOLERANCE {
                    faces.push(face);
                    (origin_to_closest_plane_offset, closest_plane, faces)
                } else {
                    (origin_to_closest_plane_offset, closest_plane, faces)
                }
            });

        let unique_face_indices = faces.iter()
            .map(|face| {
                (&self.support_points[face.0], &self.support_points[face.1], &self.support_points[face.2])
            })
            .fold((HashSet::new(), HashSet::new()), |mut sets, face_support_points| {
                sets.0.insert(((face_support_points.0).1).0);
                sets.0.insert(((face_support_points.1).1).0);
                sets.0.insert(((face_support_points.2).1).0);

                sets.1.insert(((face_support_points.0).1).1);
                sets.1.insert(((face_support_points.1).1).1);
                sets.1.insert(((face_support_points.2).1).1);

                return sets;
            });

        let feature_types = (
            Polytope::infer_feature_type(unique_face_indices.0),
            Polytope::infer_feature_type(unique_face_indices.1),
        );

        return match feature_types {
            (Feature::Vertex(index), _other) => {
                let correction = closest_plane.normal() * penetration_depth / 2.0;
                let contact_point = self.diff.0.vertex(index) + correction;
                let contact_normal = -closest_plane.normal();

                ContactSet::new(
                    Plane::new(contact_point - contact_normal * penetration_depth, contact_normal),
                    vec!(contact_point),
                )
            },

            (_other, Feature::Vertex(index)) => {
                let correction = closest_plane.normal() * penetration_depth / 2.0;
                let contact_point = self.diff.1.vertex(index) - correction;
                let contact_normal = -closest_plane.normal();

                ContactSet::new(
                    Plane::new(contact_point - contact_normal * penetration_depth, contact_normal),
                    vec!(contact_point),
                )
            },

            (Feature::Edge(edge_0), Feature::Edge(edge_1)) => {
                let vertex_00 = self.diff.0.vertex(edge_0.0);
                let vertex_01 = self.diff.0.vertex(edge_0.1);
                let vertex_10 = self.diff.1.vertex(edge_1.0);
                let vertex_11 = self.diff.1.vertex(edge_1.1);

                let line_0 = Line::from_points(vertex_00, vertex_01);
                let line_1 = Line::from_points(vertex_10, vertex_11);

                let contact_point = line_0.closest_point_to_line(&line_1);
                let contact_normal = -closest_plane.normal();

                ContactSet::new(
                    Plane::new(contact_point - contact_normal * penetration_depth, contact_normal),
                    vec!(contact_point),
                )
            },

            (Feature::Edge(edge), Feature::Face(face_indices)) => {
                Polytope::compute_contact_set_for_edge_face(&self.diff, closest_plane, edge, face_indices)
            },

            (Feature::Face(face_indices), Feature::Edge(edge)) => {
                Polytope::compute_contact_set_for_edge_face(&self.diff.clone().reversed(), closest_plane, edge, face_indices)
            },

            (Feature::Face(face_0_indices), Feature::Face(face_1_indices)) => {
                Polytope::compute_contact_set_for_face_face(&self.diff, closest_plane, face_0_indices, face_1_indices)
            },
        };
    }

    fn infer_feature_type(indices: HashSet<usize>) -> Feature {
        match indices.len() {
            0 => panic!("No vertices describing the contact feature"),

            1 => {
                Feature::Vertex(indices.into_iter().next().unwrap())
            },

            2 => {
                let mut iterator = indices.into_iter();
                let index_0 = iterator.next().unwrap();
                let index_1 = iterator.next().unwrap();

                Feature::Edge((index_0, index_1))
            },

            _otherwise => Feature::Face(indices),
        }
    }

    fn compute_contact_set_for_edge_face(diff: &MinkowskiDifference, contact_plane: Plane, edge_indices: (usize, usize), face_indices: HashSet<usize>) -> ContactSet {
        let coordinates = CoordinateTransform::from_plane(&contact_plane);

        let start = coordinates.transform(diff.0.vertex(edge_indices.0));
        let end = coordinates.transform(diff.0.vertex(edge_indices.1));
        let average_z_0 = (start.z + end.z) / 2.0;
        let edge_points = Line2D::new(
            Vec2D::new(start.x, start.y),
            Vec2D::new(end.x, end.y),
        );

        let (polygon, average_z_1) = project_to_polygon_2d(diff.1, face_indices, &coordinates);
        let average_z = (average_z_0 + average_z_1) / 2.0;

        let intersection = polygon.intersection(&edge_points).unwrap();
        let contact_point_0 = coordinates.transform_with_inverse(Vec3D::new(intersection.start.x, intersection.start.y, average_z));
        let contact_point_1 = coordinates.transform_with_inverse(Vec3D::new(intersection.end.x, intersection.end.y, average_z));

        return ContactSet::new(
            Plane::new(contact_point_0, -contact_plane.normal()),
            vec!(contact_point_0, contact_point_1),
        );
    }

    fn compute_contact_set_for_face_face(diff: &MinkowskiDifference, contact_plane: Plane, face_0_indices: HashSet<usize>, face_1_indices: HashSet<usize>) -> ContactSet {
        let coordinates = CoordinateTransform::from_plane(&contact_plane);

        let (polygon_0, average_z_0) = project_to_polygon_2d(diff.0, face_0_indices, &coordinates);
        let (polygon_1, average_z_1) = project_to_polygon_2d(diff.1, face_1_indices, &coordinates);
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
}

fn project_to_polygon_2d(form: &Form, indices: HashSet<usize>, coordinates: &CoordinateTransform) -> (Polygon, Scalar) {
    let points: Vec<Vec3D> = indices.into_iter()
        .map(|index| coordinates.transform(form.vertex(index)))
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
