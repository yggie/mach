use std::collections::HashSet;

use {NEG_INFINITY, TOLERANCE};
use maths::{DotProduct, Vec3D};
use maths::_2d::Vec2D;
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
        let dummy_plane = Plane::from_point(&Vec3D::new(1.0, 0.0, 0.0), &Vec3D::new(0.0, 0.0, 0.0));

        let faces = vec!((0, 0, 0));
        let (penetration_depth, closest_plane, faces) = self.surfaces.iter().skip(1)
            .fold((NEG_INFINITY, dummy_plane, faces), |(origin_to_closest_plane_offset, closest_plane, mut faces), &(ref plane, face)| {
                let offset = plane.offset_for_origin();
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
                    Plane::from_point(&(contact_point - contact_normal * penetration_depth), &contact_normal),
                    vec!(contact_point),
                )
            },

            (_other, Feature::Vertex(index)) => {
                let correction = closest_plane.normal() * penetration_depth / 2.0;
                let contact_point = self.diff.1.vertex(index) - correction;
                let contact_normal = -closest_plane.normal();

                ContactSet::new(
                    Plane::from_point(&(contact_point - contact_normal * penetration_depth), &contact_normal),
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
                    Plane::from_point(&(contact_point - contact_normal * penetration_depth), &contact_normal),
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
                panic!("Got an unexpected FACE|FACE contact, face_0: {:?}, face_1: {:?}", face_0_indices, face_1_indices);
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
        let projected_x_axis = contact_plane.normal().cross(/* TODO pick a random vector */Vec3D::new(1.0, 1.0, 1.0)).normalize();
        let projected_y_axis = contact_plane.normal().cross(projected_x_axis).normalize();
        let project = |point: &Vec3D| -> Vec2D {
            Vec2D::new(
                projected_x_axis.dot(*point),
                projected_y_axis.dot(*point),
            )
        };

        let unproject = |point: &Vec2D| -> Vec3D {
            point.x * projected_x_axis + point.y * projected_y_axis
        };

        // TODO check depth correction
        let depth_0 = diff.0.vertex(edge_indices.0).dot(contact_plane.normal().clone());
        let depth_1 = diff.0.vertex(edge_indices.1).dot(contact_plane.normal().clone());

        let edge_points = Line2D::new(
            project(&diff.0.vertex(edge_indices.0)),
            project(&diff.0.vertex(edge_indices.1)),
        );
        // TODO remove clone once the panic! is gone
        let face_points: Vec<Vec2D> = face_indices.clone().into_iter()
            .map(|index| project(&diff.1.vertex(index)))
            .collect();

        let polygon = Polygon::convex_hull_from(&face_points)
            // TODO can we avoid needing to validate this?
            .expect("A valid face always has enough points");

        let intersection = polygon.intersection(&edge_points).unwrap();
        let contact_point_0 = unproject(&intersection.start) + depth_0 * contact_plane.normal();
        let contact_point_1 = unproject(&intersection.end) + depth_1 * contact_plane.normal();

        return ContactSet::new(
            Plane::from_point(&contact_point_0, &-contact_plane.normal()),
            vec!(contact_point_0, contact_point_1),
        );
    }
}
